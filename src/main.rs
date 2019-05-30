extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Please read in a number:");
    let mut guess = String::new(); //We read in a string here
    io::stdin().read_line(&mut guess) //This is how to read in a string
        .expect("Failed to read");
    let guess: u32 = guess.trim().parse()
        .expect("Please enter a number"); //Shadow value. The main purpose of having shadow values is for type conversion.

    let secret_number = rand::thread_rng().gen_range(1,101); //Random number generator.

    println!("The secret number is: {}", secret_number);

    println!("You've guessed: {}",guess);
    //This function is from the cmp io. Comparing the first operand (guess) with the argument passed in by reference
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small."),
        Ordering::Equal => println!("Match."),
        Ordering::Greater => println!("Too large."),
    }
}

