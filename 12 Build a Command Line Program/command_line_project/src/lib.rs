use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // function name changed, so it makes sense when called (Config::new(args) -> create new config using these arguments).
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // There is multiple ways of handling errors and result is more user friendly than panicing.
            // panic!("not enough arguments");

            // Remember Result can be Ok or Err.
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // and because the function returns Result Config needs to be wrapped.
        Ok(Config { query, file_path })
    }
}

// Seperating the logic in a way that it can be moved in another file
// Error handling in a user friendly way (aka not panicing with expect method)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // Return Ok if everything is processed correctly.
    // Because end product is just a println Ok doesn't need to contain any other
    // thing other than "()" which is specified as a Result Ok value.
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Old implementation, implemented in Config struct
// For this sized project, separating parse functionality like this might be too much.
// fn parse_config(args: &[String]) -> Config {
//     // Make identical copy of the &args, which takes more resources,
//     // but we don't need to think ownerships and lifetimes.
//     let query = &args[1].clone();
//     let file_path = &args[2].clone();

//     // Oh and Config would break Rust rules at this point without the clone methods.
//     // Ofcourse there are other ways of correcting the issue other than clone method.
//     // Chapter 13 introduces more efficient to solve the ownership problem.
//     Config { query, file_path }
// }