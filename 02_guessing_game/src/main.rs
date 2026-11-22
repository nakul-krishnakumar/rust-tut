use std::io::{self, Write};
use std::cmp::Ordering;
use rand::RngExt;


fn main() {
    println!("{} Welcome to Guessing Game! {}", "=".repeat(10), "=".repeat(10));

	let rnum: u8 = rand::rng().random_range(1..=100);
	// print!("Random number is: {rnum}");

	const MAX_ATTEMPTS: u8 = 5;
	let mut attempts: u8 = 0;
	let mut guess: String = String::new(); // mut means the variable is mutable
	// In Rust, all variables are immutable by default, unless `mut` keyword is added


	while attempts < MAX_ATTEMPTS {
		guess.clear();
		attempts += 1;

		print!("\tEnter your guess: ");
		io::stdout().flush().unwrap();
		/*
		* Why `io::stdout().flush().unwrap();`?
		* - Using `print!()` means it won't write a newline
		* - In these cases Rust don't immediately write it to stdout
		* - It waits for either a newline to come or the program to end,
		* - Then writes all of them together.
		* - `io::stdout().flush().unwrap();` forces Rust to flush to stdout
		*/


		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		let guess: u8 = match guess
			.trim()
			.parse::<u8>() {
				Ok(num) => num,
				Err(_) => continue
			};
		/*
		- trim() -> trims leading and trailing whitespaces, \t,\n and similar tags
		- parse() -> converts the string slice into another type:
			- guess: u8 = guess.trim().parse().expect(&msg);
			- guess = guess.trim().parse::<u8>().expect(&msg);
		 */
		
		match guess.cmp(&rnum) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You guessed it correct!\nYou took {} attempts!", attempts);
				break;
			}
		}
		if attempts >= MAX_ATTEMPTS { println!("You are out of attempts! :(\nCorrect number is {rnum}"); }
	}
}