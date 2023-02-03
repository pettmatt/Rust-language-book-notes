use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // simple_error_handler();
    // advanced_error_handler();
    propagating_errors();
}

fn simple_error_handler() {
    // File::open handles errors by returning `Result<T, E>`, which can hold either Ok or Err
    // Ok contains file handle and Err contains information about the error
    let greeting_file_result = File::open("hello.txt");

    // With the Result within greeting_file_result we can compare the value with Ok and Err state.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Probably not the best way of handling error that can be fixed by adding a file
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn advanced_error_handler() {
    let greeting_file_result = File::open("hello.txt");

    // In this match expression the program checks if the result state is Ok or Err, just like before.
    // After that if it's an error the program checks what kind of an error it is and resolves it.
    // For more details about this code read explonation from the book.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Nesting match expressions is legal move
        Err(error) => match error.kind() {
            // https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.NotFound
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn advanced_error_handler_using_if_statements() {
    // If statements can be more readable in cases where the application requires nesting.
    // Unwrap_or_else is method from std library, which handles the result state Err or returns Ok
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn propagating_errors() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Exits early from the function
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn propagating_errors_advanced() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}