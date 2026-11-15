use std::io::{self, Write};
use rand::RngExt;


fn main() {
    println!("{} Welcome to Guessing Game! {}", "=".repeat(10), "=".repeat(10));

	let rnum: i8 = rand::rng().random_range(1..=100);
	print!("Random number is: {rnum}");

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

	let mut guess: String = String::new(); // mut means the variable is mutable
	// In Rust, all variables are immutable by default, unless `mut` keyword is added
	
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	let guess: i8 = guess.trim().parse().expect("Invalid number!");

	println!(
		"You guessed it {}", 
		if guess == rnum {
			"correct!".to_string()
		} 
		else {
			format!("wrong!\nThe number was {rnum}")
		}
	);

	// println!("\tYou guessed: {guess} and 10 + 2 = {}", 10+2);
}