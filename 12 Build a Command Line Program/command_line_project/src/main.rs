use std::env;
use std::fs;

fn main() {
    // get the arguments that were passed through command line
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("In file {}", file_path);
    println!("Searching for {}", query);

    // Read a file
    let contents = fs::read_to_string(file_path)
        .expect("Error occured while reading a file");

    println!("Contents of the file:\n{}", contents);
}
