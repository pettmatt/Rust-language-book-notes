fn main() {
    println!("Hello, world!");
    example_one();
    example_two(0);

    println!("------");
    example_three();
}

fn example_one() {
    println!("This is function which was called by main.");

    println!("Rust doesn't really care when or where function is created.");
    println!("If Rust file has access to it, it will be executed after the call has been made.");
}

fn example_two(x: i32) {
    println!("This one is little different.");
    println!("This one can receive arguments through parameter.");
    println!("For example value '{x}' can be manipulated by giving this function an argument.");
}

fn example_three() {
    // No. It's not an object.
    // This here is called a scope block, which is an expression.
    let y = {
        let x = 3;
        x + 1 // Doesn't include ";", that would turn this expression into a statement.
    };

    let x = example_four(3);

    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
}

fn example_four(x: i32) -> i32 {
    // In rust using a return keyword isn't required and ending a file
    // like "x + 1" is completely fine, this returns the value of 4.
    // Note that the function is declaring the return type with "-> i32".
    x + 1
    // Adding ";" would throw an error, because the function isn't returning a number.
    // And why isn't it returning? Because statements don't return values. They just state the statements.
}