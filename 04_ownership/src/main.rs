
///////////////// OWNERSHIP /////////////////


fn main() {
	// ? What is Ownership?
	// * Ownership is a discipline for ensuring the safety of Rust programs.
    // * It enables Rust to make memory safety guarantees without needing a `garbage collector`

	// read(x); // oh no! x isn't defined!
    // let x = true;

	// * VARIABLES LIVE IN THE STACK
	variables_live_in_the_stack();

	// * BOXES LIVE IN THE HEAP
	boxes_live_in_the_heap();

	// * RUST DOES NOT PERMIT MANUAL MEMORY MANAGEMENT
	rust_doesnt_permit_manual_memory_management();

	// * COLLECTIONS USES BOXES
	// ? Boxes are used by Rust data structures like Vec, String, and HashMap
	// ?    to hold a variable number of elements.

	// * OWNERSHIP EXAMPLE
	ownership_example();

	// * HOW TO AVOID MOVES? -> CLONES
	cloning_example();

	// !!! SUMMARY
	/*
	Ownership is primarily a discipline of heap management:
    - All heap data must be owned by exactly one variable.
    - Rust deallocates heap data once its owner goes out of scope.
    - Ownership can be transferred by moves, which happen on assignments and function calls.
    - Heap data can only be accessed through its current owner, not a previous owner.
	 */
}

// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

fn variables_live_in_the_stack() {
	// Variables live in frames.
	// A frame is a mapping from variables to values within a single scope,
	// such as a function. For example:
	/*
		fn main() {
			let n = 5;
			let y = plus_one(n);
			println!("The value of y is: {y}");
		}

		fn plus_one(x: i32) -> i32 {
			x + 1
		}	
	*/
	// - The frame for main at location L1 holds n = 5.
	// - The frame for plus_one at L2 holds x = 5.
	// - The frame for main at location L3 holds n = 5; y = 6. 

	// * Frames are organized into a stack of currently-called-functions.
	// * After a function returns, Rust deallocates the function’s frame.
	// *   (Deallocation is also called freeing or dropping)
}

fn boxes_live_in_the_heap() {

	// - Using arrays
	/*
		let a = [0; 1_000_000];  // L1
		let b = a;               // L2

	- Here, `a` is a normal array, and is kept in stack
	- so copying `a` into `b` is very costly.

	STACK AT L1 : 
		main(
			[a, [0, 0, 0, ...., 0]],
		)
	STACK AT L2 : 
		main(
			[a, [0, 0, 0, ...., 0]],
			[b, [0, 0, 0, ...., 0]],
		)
	
	- * Here copying `a` into `b` causes the main frame to contain 2 million elements.

	*/

	// Using Boxed arrays (they live in heap)
	/*
		let a = Box::new([0; 1_000_000]);   // L1
		let b = a;                          // L2

	STACK AT L1 : 
		main(
			[a, pointer to h1 kept in heap]
		)
	HEAP AT L1:
		h1: [0,0,0,0,...,0]

	STACK AT L2 :
		main(
			[a, pointer to h1 kept in heap],
			[b, pointer to h1 kept in heap]
		)
	HEAP AT L2:
		h1: [0,0,0,0,...,0]

	- * Observe that now, there is only ever a single array at a time. 
	- * At L1, the value of a is a pointer to the array inside the heap.
	- * The statement let b = a copies the pointer from a into b, but the
	-   pointed-to data is not copied.
	 */
}

fn rust_doesnt_permit_manual_memory_management() {
	/*
	- * Memory management is the process of allocating memory and deallocating memory.
	- * Stack frames are automatically managed by Rust.
	- * When a function is called, Rust allocates a stack frame for the called function.
        When the call ends, Rust deallocates the stack frame.
	- * But when is heap data deallocated? Rust doesnt allow a free() method to manually deallocate mem.

	- * A Box’s Owner Manages Deallocation
	 */

	// * Rust automatically frees a box’s heap memory.
	// ? Box deallocation principle (almost correct): If a variable is bound to a box,
	// ?   when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

	// ? OWNERSHIP
	// * - When a is bound to Box::new([0; 1_000_000]), we say that a owns the box.
	// * - The statement let b = a moves ownership of the box from a to b.
	// * - Given these concepts, Rust’s policy for freeing boxes is more accurately described as:
	/*
	Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates
	the variable’s frame, then Rust deallocates the box’s heap memory.
	 */
}

// Here, the ownership of the string changes to `full_name` from `first_name`,
// 1. Initially `first_name`` is a variable kept in stack, which points to "Nakul" value kept in Heap
// 2. Then `name` parameter in `add_suffix` fn is also pointed to "Nakul" value kept in Heap
// 3. When `name` is pointed to "Nakul", it gets its OWNERSHIP and `first_name` loosed the OWNERSHIP
// 4. Value "Nakul" is updated to "Nakul Krishnakumar" through the `name` variable pointer
// 5. Returning name transfers ownership to full_name.
// 6. Printing first_name causes a compile-time error because
//    its value was moved, not because it was uninitialized.
//
// * Moved heap data principle: if a variable x moves ownership of heap data to another
// *    variable y, then x cannot be used after the move.
fn ownership_example() {
	let first_name = String::from("Nakul");
	let full_name = add_suffix(first_name);

	println!("{} is the full name of {}", full_name, first_name);
}

fn cloning_example() {
	let first = String::from("Nakul");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
	name.push_str(" Krishnakumar");
	name
}