extern crate rand;

use rand::prelude::*;

fn main() {

    let mut total_draws : i64 = 0;
    let mut finished_rounds : i32 = 0;

    //Init rng for shuffiling the first order of the deck, and for shuffling aces back into the deck
    let mut rng = thread_rng();

    let mut counts_of_draws : Vec<i32> = vec![0; 49];

    let mut iterations = 300_000;

    while iterations > 0 {
        iterations -= 1;

        //Initialize vector
        let mut cards : Vec<i8> = (1..53).collect();

        //Shuffle the original opsition of the deck
        cards.shuffle(&mut rng);
        
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
                let index = (index_float.floor() + 1f64) as usize; 
                cards.insert(index, current_card); //Inserting the drawn ace back into the deck at a random position
            }
        }

        //Updating the overall variables, so that we can keep track of the statistics
        total_draws += i64::from(draws);
        finished_rounds += 1;
        counts_of_draws[(draws-4) as usize] += 1;

        //Printing the statistics out to the console
        //println!("This rounds draws: {}", draws);
        //println!("Rounds still left to go: {}\n", iterations);
        //Prints way too fast to be readable in this version, so disabled, you can enable them by removing the commnent ("//")
        
        //Print current progress every thousand loops
        if iterations % 1000 == 0 {
            println!("Loops still to be done: {} \n", iterations);
        }
    };

    println!("Overall we did {} rounds of drawing.", finished_rounds);
    println!("And in those rounds made {} draws\n", total_draws);

    //Calculate average number of draws
    let total_draws_float : f64 = total_draws as f64;
    let finished_rounds_float : f64 = f64::from(finished_rounds);

    let average_number_of_draws : f64 = total_draws_float / finished_rounds_float;

    println!("So on average you need to draw {} cards to get 4 aces cards with this setup. \n", average_number_of_draws);
    
    println!("From the iterations run this time here's how many draws to get the aces and how many times they occured:");
    for (i, occurance) in counts_of_draws.iter().enumerate().take(49) {
        println!("Draws {number_of_draws}: {occurances}", number_of_draws = (i+4), occurances = occurance )
    }
}
