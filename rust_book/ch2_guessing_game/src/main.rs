// import io library from the standard library
use std::io;

// import Ordering enum from the standard library
use std::cmp::Ordering;

// import random number generator trait from rand library crate
use rand::Rng;

// main entry-point
fn main() {
    // print text to the user
    println!("Guess a number!");

    // generate secret random number from 1-100 inclusive
    // and infer the value as a u32 type
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        // create mutable variable bound to an empty string (immutable by default)
        let mut guess = String::new();

        println!("Please input your guess:");

        // Read a line of text from stdin and store the text into the
        // mutable string variable reference passed to the function.
        // Check the Result enum type returned by the read_line() function
        // and expect it to not be an error (exiting and printing the
        // given error message if an error occurs).
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the user's guess string to a u32 value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // print the user's guess
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
