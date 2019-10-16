extern crate reqwest;

use reqwest::Error;

fn main() -> Result<(), Error> {
    //Hardcoded quota string to check if we still have bits of randomness we can use 
    let quota_url = String::from("https://www.random.org/quota/?format=plain");

    let mut quota_response = reqwest::get(&quota_url)?;

    //Move into it's own string object, and remove leading and trailing whitespace, so that parsing doesn't fail
    let mut quota_text = quota_response.text()?;
    quota_text = quota_text.trim().to_string();

    //Try to parse to an int, if fails something else is wrong and we should exec further so we panic
    let quota = quota_text.parse::<i32>().unwrap();
    
    //println!("{}", quota_text);

    if quota < 1613 {panic!("Not enough bits of randomness available for randomizing deck");};

    let request_url = format!("https://www.random.org/sequences/?min={min}&max={max}&col={colums}&format={format}&rnd={randomMethod}",
                    min = 1,
                    max = 52,
                    colums = 1,
                    format = "plain", //Either plain for plaintext, or html for html formated normal page. Plain is easier to code for.
                    randomMethod = "today"     //Options are, "new" for using new bits, or date to use pregenerated randomness for that day
    
    );

    let mut response = reqwest::get(&request_url)?;

    let mut cards : Vec<i8>;

    let response_text : String = response.text()?;

    //println!("\n{}", response_text);

    Ok(())
}
