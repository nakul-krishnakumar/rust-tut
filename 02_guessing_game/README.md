What this code has:
- loop, while
- match
- x.cmp()
- parse(), trim()
- stdout().flust().unwrap()
- let rnum: u8 = rand::rng().random_range(1..=100);


```rust
// 1. loop: unconditional repetition
loop {
    // Runs until break
}

// 2. while: condition-based repetition
while condition {
    // Runs while condition is true
}

// 3. for: iterate over a range or collection
for i in 1..=5 {
    println!("{i}");
}
```
```rust
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
```
```rust
		let guess: u8 = guess
			.trim()
			.parse::<u8>()
			.expect("Invalid number!");
		/*
		- trim() -> trims leading and trailing whitespaces, \t,\n and similar tags
		- parse() -> converts the string slice into another type:
			- guess: u8 = guess.trim().parse().expect(&msg);
			- guess = guess.trim().parse::<u8>().expect(&msg);
		 */
```