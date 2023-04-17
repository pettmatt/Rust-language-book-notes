use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Get the arguments that were passed through command line.
    let args: Vec<String> = env::args().collect();

    // Implementing the parse_config function through config struct might be better, because Config struct is only used in context of parse_config.
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Handling possible errors.
        // Note that the error is processed by the standard library error print macro.
        eprintln!("Problem parsing arguments: {err}");
        // The process::exit function will stop the program immediately and return the number that was passed as the exit status code
        process::exit(1);
    });

    // Commented out, so the output.txt is more readable.
    // println!("In file {}", config.file_path);
    // println!("Searching for {}", config.query);

    // Read the file.
    if let Err(e) = minigrep::run(config) {
        // Because error can be returned it needs to be handled.
        println!("Application error: {e}");
        process::exit(1);
    }
}