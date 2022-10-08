use std::io;

fn main() {
    example_one();
    example_two();
    book_one();
}

fn example_one() {
    //let guess; // WRONG
    //let guess: _; // Prompts "type annotations needed" error
}

fn example_two() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring tuple

    println!("The value of y is: {y}");
}

fn book_one() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}