
fn main() {
	// 1. Shadowing
	_shadowing();

	// 2. Tuples
	_tuples();

	// 3. Arrays
	_arrays();
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

	let x: [i32; 5] = [1,2,3,4,5];

	let a = [3; 5]; // !!! Create an Array of size 5, filled with all 3s.
	println!("{:?}", a); // * [3, 3, 3, 3, 3] * //

	println!("{}", a[5]);

}