fn main() {
    // Normal syntax and language logic.
    // Before declaring a variable, it's not valid (in scope, but not declared).
    let v = "I am valid";

    // Now variable "v" is valid (and in scope) and can be used.
    println!("{v}");

    example_one();
} // Now we're out of scope, so the variable is not valid.

// After the scope the used memory is automatically freed.
// In this case the memory used by "s" and "v" variables is freed.
// Rust has build in function which handles this operation. It's called "drop".

fn example_one() {
    // When String::from() is executed, it requests the system to give more resources to the program.
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);
}

