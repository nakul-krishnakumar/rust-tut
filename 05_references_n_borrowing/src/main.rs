
// * DEMONSTRATION OF THE MAIN ISSUE OF MOVE-ONLY APIs
fn main() {
	issue_with_move_only_apis();

	// * SOLUTION - REFERENCES & BORROWING
	references_n_borrowing();

	// Dereferencing a Pointer Accesses Its Data
	dereferencing_a_pointer();

	// * Aliasing & Mutation
	// - Aliasing: Accessing the same data through different variables (pointers).
	// - On its own, aliasing is harmless. But combined with mutation, we have a recipe for disaster
	// - Can lead to dangling pointers
	
}

fn issue_with_move_only_apis() {
	// ! ISSUE
    let m1 = String::from("Nakul");
	let m2 = String::from("Krishnakumar");

	// _greet(m1, m2);
	// println!("{}, {}", m1, m2); // The main issue of move-only APIs
}

fn references_n_borrowing() {
	let m1 = String::from("Nakul");
	let m2 = String::from("Krishnakumar");

	// Here the expression &m1 uses the ampersand operator to create a
	//  reference to (or “borrow”) m1
	// References Are Non-Owning Pointers, because they do not own the data they point to.
	_better_greet(&m1, &m2);
	println!("{}, {} - WORKED!", m1, m2); // SOLVED!!!
}

fn dereferencing_a_pointer() {
	let mut x = Box::new(5);
	// x owns a heap-allocated i32 containing 5.
	// x itself is typically stored on the stack as a pointer.

	let a = *x;
	// a gets a COPY of the value 5, typically stored on the stack.
	// This works because i32 implements the Copy trait.

	// let b = x;
	// Ownership of the Box moves from x to b.
	// b now points DIRECTLY to the same heap-allocated i32.
	// x is no longer valid.

	*x += 1; // Now `x` = 6, but `a` = 5 still as `a` is not a pointer, but a value kept in stack
}

fn _greet(g1: String, g2: String) {
	println!("{}, {}", g1, g2);
}

fn _better_greet(g1: &String, g2: &String) {
	// The type of the greet parameter g1 is changed to &String,
	//  meaning “a reference to a String”.
	println!("{}, {}", g1, g2);
}

