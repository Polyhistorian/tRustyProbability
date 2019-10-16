extern crate serde;
extern crate reqwest;

use serde::{Serialize, Deserialize};
use reqwest::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let quota_url = String::from("https://www.random.org/quota/?format=plain");

    let mut quota_response = reqwest::get(&quota_url)?;

    let request_url = format!("https://www.random.org/sequences/?min={min}&max={max}&col={colums}&format={format}&rnd={randomMethod}",
                    min = 1,
                    max = 52,
                    colums = 1,
                    format = "plain", //Either plain for plaintext, or html for html formated normal page. Plain is easier to code for.
                    randomMethod = "today"     //Options are, "new" for using new bits, or date to use pregenerated randomness for that day
    
    );

    println!("{}", request_url);

    let mut response = reqwest::get(&request_url)?;

    let mut cards : Vec<i8>;

    let response_text : String = response.text()?;



    println!("\n{:?}", response_text);

    Ok(())
}
