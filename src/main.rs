/*
Author: Constantine Lekkos
Purpose: Official Rust Tutorial Book
Created Date: 28/10/2026
*/

use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

/*
This main function is the entry point of the program.
It generates a random number 1-100, the user enters a value,
and in case the value is not an integer, then forces the user
to enter a new value in a loop. Then the input value is compared
to the secret number and outputs the result. Have fun!
*/
fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
