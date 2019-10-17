extern crate reqwest;
extern crate rand;

use reqwest::Error;
use reqwest::header::USER_AGENT;
use rand::prelude::*;

fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    //Hardcoded quota string to check if we still have bits of randomness we can use 
    let quota_url = String::from("https://www.random.org/quota/?format=plain");

    let mut quota_response = client.get(&quota_url)
        .header(USER_AGENT, "tRustyProbability - thefinnisharmy@gmail.com")
        .send()?;

    //Move into it's own string object, and remove leading and trailing whitespace, so that parsing doesn't fail
    let mut quota_text = quota_response.text()?;
    quota_text = quota_text.trim().to_string();

    //Try to parse to an int, if fails something else is wrong and we should exec further so we panic
    let mut quota = quota_text.parse::<i32>().unwrap();
    
    //println!("{}", quota_text);

    if quota < 1613 {panic!("Not enough bits of randomness available for randomizing deck");};

    let mut total_draws : i64 = 0;
    let mut finished_rounds : i32 = 0;

    let mut counts_of_draws : Vec<i32> = vec![0; 49];

    

    //Init rng for shuffling all aces cards back into the deck, worse rng than random.org, but using it here for saving bits on the requests 
    //(Plus makes handling still available bits easier)
    let mut rng = thread_rng();

    while quota > 1613 {
        //Create url for the https request, and then send it
        let request_url = format!("https://www.random.org/sequences/?min={min}&max={max}&col={colums}&format={format}&rnd={randomMethod}",
                    min = 1, //Minium value to be returned, for a card deck should be 1
                    max = 52, //Maximum value to be returned, for a card deck should be 52
                    colums = 1,
                    format = "plain", //Either plain for plaintext, or html for html formated normal page. Plain is easier to code for.
                    randomMethod = "today"     //Options are, "new" for using new bits, or date to use pregenerated randomness for that day
                    );

        let mut response = client.get(&request_url)
            .header(USER_AGENT, "tRustyProbability - thefinnisharmy@gmail.com")
            .send()?;

        //Calculate still available quota, to check whether we can do anoter round
        quota -= 1612;

        //Move response body into its own variable
        let cards_text : String = response.text()?;

        //Initialize vectors
        let mut split_card_text : Vec<&str> = Vec::with_capacity(52);
        let mut cards : Vec<i8> = Vec::with_capacity(52);

        //Split the body into discrete "numbers"
        let mut cards_text_vector = cards_text.split_whitespace();

        //Iterate over those strings pushing them into a vector, while also converting them into an interger
        for i in 0..52 {
            split_card_text.push(cards_text_vector.next().unwrap());
            cards.push(split_card_text[i].parse::<i8>().unwrap());
        }
        
        //Loops variables to track their names
        let mut draws : i32 = 0;
        let mut found_aces : i32 = 0;
        
        //"Drawing" one value from the deck continuously until we reach 4 aces cards
        while found_aces < 4 {
            draws += 1;
            let current_card = cards.pop().unwrap(); //Picks the last card from the vector, doesn't matter statistically whether we start from the beginning or end
            if current_card <= 4 { //Matching all aces, which have the chronological numbers 1-4
                found_aces += 1;
                let mut index_float : f64 = rng.gen(); //Picking a random float (decimal) between 0-1
                index_float *= cards.len().to_string().parse::<f64>().unwrap(); //Multiplying it by the current length of the vector, so that it doesn't end up outside of it
                let index = index_float.round() as usize; 
                cards.insert(index, current_card); //Inserting the drawn ace back into the deck at a random position
            }
        }

        //Updating the overall variables, so that we can keep track of the statistics
        total_draws += i64::from(draws);
        finished_rounds += 1;
        
        counts_of_draws[(draws-4) as usize] += 1;

        //Printing the statistics out to the console
        println!("This rounds draws: {}", draws);
        println!("Still available random bits: {} \n \n", quota);
    };

    println!("Overall we did {} rounds of drawing.", finished_rounds);
    println!("And in those rounds made {} draws\n", total_draws);

    //Calculate average number of draws
    let total_draws_float : f64 = total_draws as f64;
    let finished_rounds_float : f64 = f64::from(finished_rounds);

    let average_number_of_draws : f64 = total_draws_float / finished_rounds_float;

    println!("So on average you need to draw {} cards to get 4 aces cards with this setup.", average_number_of_draws);

    println!("From the iterations run this time here's how many draws to get the aces and how many times they occured:");
    for (i, occurance) in counts_of_draws.iter().enumerate().take(49) {
        println!("Draws {number_of_draws}: {occurances}", number_of_draws = (i+4), occurances = occurance )
    }

    //Returning Ok, is here so that error handling otherwhere is easier
    Ok(())
}
