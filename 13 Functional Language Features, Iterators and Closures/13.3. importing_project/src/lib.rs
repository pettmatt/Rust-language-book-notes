use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // The parameter type has been modified, now it's mutable, that implements Iterator with a String value
    pub fn build(mut args: impl Iterator<Item = String>)-> Result<Config, &'static str> {
        // Iterate once so the variables get correct values
        args.next();

        // Iterators can populate variables, without then need of cloning method (which somewhat slower)
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        // Earlier the ownership wasn't guaranteed which is why this part included if-statement
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// For loop replaced with iterator, which makes the code cleaner and probably even easier to read.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // This method also removes the need of Vector variable which is then returned after the for loop.
    // The iterator in use is an adaptor (if we want to be more precise).
    contents
        .lines() // Seperate the string into lines.
        .filter(|line| line.contains(query)) // We only want to return lines that contain the "query" value.
        .collect() // Consume the line that manages to survive the filter condition.

    // The final product is a Vector can contains string values, just like before.
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}