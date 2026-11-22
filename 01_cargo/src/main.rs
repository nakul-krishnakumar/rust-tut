fn main() {
    println!("Hello, world!");
}

/*
| Function                | Macro                         |
| ----------------------- | ----------------------------- |
| Runs at runtime         | Expands at compile time       |
| Called with `()`        | Called with `!()`             |
| Receives values         | Receives Rust syntax (tokens) |
| Cannot change your code | Generates new Rust code       |
| Has fixed parameters    | Can accept flexible syntax    |
*/

// $ cargo new project_1
// $ cargo new 02_cargo  -> wont work, instead use `cargo new 02_cargo --name cargo_02`