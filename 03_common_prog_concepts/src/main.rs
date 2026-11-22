
fn main() {
	// 1. Shadowing
	_shadowing();

	// 2. Tuples
	_tuples();

	// 3. Arrays
	_arrays();

	// 4. Functions
	_functions();

	// 5. Branching (IF)
	_branching();

	// 6. Loops (for, loop, while)
	_loops();
}

///////////////// SHADOWING /////////////////

fn _shadowing() {
    let x = 5;

    let x = x + 1; // shadowing

    {
        let x = x * 2; // shadowing
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

/*
OUTPUT:
	$ cargo run
	Compiling variables v0.1.0 (file:///projects/variables)
		Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
		Running `target/debug/variables`
	The value of x in the inner scope is: 12
	The value of x is: 6
*/

///////////////// TUPLES /////////////////

fn _tuples() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup; // !!! DESTRUCTING !!!

    println!("The value of y is: {y}");

	// !!! TUPLE ELEMENT ACESSING !!! //
	let mut x: (i32, f64, u8) = (500, 6.4, 1);
	let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
	x.2 = 30; // !!! TUPLE CAN BE MADE MUTABLE
	println!("{:?}", x);
}

///////////////// ARRAYS /////////////////

fn _arrays() {
    /*
	- Arrays in Rust are of fixed size.
	- Arrays are useful when you want your data allocated on the stack. (tuples as well)
	 */

	let _x: [i32; 5] = [1,2,3,4,5];

	let a = [3; 5]; // !!! Create an Array of size 5, filled with all 3s.
	println!("{:?}", a); // * [3, 3, 3, 3, 3] * //

	let closure_fn = |x: i8, y: i8| x + y; // !!! CLOSURE FNS !!!
	println!("{}", closure_fn(10, 11));

	// println!("{}", a[5]); // index out of bounds

}


///////////////// FUNCTIONS /////////////////

fn _functions() {
	// Two ways to return value in a rust fn

	fn five(x: i32) -> i32 {
		if x < 10 {
			return 20000; // 1. Explicit return - using return keyword (usually used for early returns)
		}

		5 // 2. Implicit return - last expression without a semicolon(;) (usual convention)
	}

	let _x = five(20);
}

// 1. Explicit return — using return
// fn five() -> i32 {
//     return 5;
// }


// - return immediately exits the function.
// - You can use it anywhere inside the function.
// - Particularly useful for early returns.
// 2. Implicit return — using the final expression
// fn five() -> i32 {
//     5
// }


// - The last expression is automatically used as the function's return value.
// - It must not have a trailing semicolon.
// - This is generally the idiomatic Rust approach.


///////////////// BRANCHING /////////////////

fn _branching() {
	// Two ways to write if conditional branches:
	// 1. `if` as control flow
	// 2. `if` as conditional expression

	// !!! NOTE: 
	// * Unlike Python, C, or C++, Rust does not support implicit truthiness.
	// * In Rust, the branching condition has to be a boolean.

	// 1. `if` as control flow
	let x = 1;
	let mut y = 2;
	if x > 2 {
		y = y + x;
	} else {
		y = y - x;
	}

	// 2. `if` as conditional expression
	let x = 1;
	let mut y = 2;
	y = if x > 2 { y + x } else { y - x };
}


///////////////// LOOPS /////////////////

fn _loops() {
	// Rust has three kinds of loops: `loop`, `while`, and `for`

	// 1. `loop` loops
	loops();

	// 2. `while` loops
	while_loops();

	// 3. `for` loops
	for_loops();
	// * Why use for loops and not while loops instead?
	// - Increased safety of the code and eliminated the chance of bugs
	//   that might result from going beyond the end of the array or not
	//   going far enough and missing some items.
}

fn loops() {
	let mut counter = 0;
	loop {
		counter += 1;
		println!("again!");

		if counter == 10 { break };
	}

	counter = 0;
	let res = loop {
		counter += 1;

		if counter == 10 {
			// Note: the semicolon after break counter * 2 is technically optional
			break counter * 2
		}
	};

	// * Loop Labels
	_demonstrate_loop_labels();
}

fn _demonstrate_loop_labels() {
	// - Loop labels must begin with a single quote.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
	let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
	
	// .rev() is reverse fn
	// (1..4) is a range fn
	for number in (1..4).rev() { 
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}