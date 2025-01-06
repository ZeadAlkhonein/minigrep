// src/lib.rs
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io;


/// Reads the entire contents of a file into a String.
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name or path of the file to read.
///
/// # Returns
///
/// Returns `String` with the content of the file or panics if the file cannot be read.
pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


/// Converts a string to a boolean value.
///
/// This function attempts to convert various string representations of boolean
/// values into their corresponding `bool`. It recognizes:
/// - "true", "yes", "on", "1" as `true`
/// - "false", "no", "off", "0" as `false`
///
/// # Arguments
///
/// * `s` - A string slice that holds the value to be converted.
///
/// # Returns
///
/// * `Some(bool)` if the string matches a recognized boolean representation.
/// * `None` otherwise.
fn string_to_bool(s: &str) -> Option<bool> {
    // Trim whitespace and convert to lowercase for case-insensitive matching
    match s.trim().to_lowercase().as_str() {
        // Matches strings that represent 'true'
        "true" | "yes" | "on" | "1" => Some(true),
        // Matches strings that represent 'false'
        "false" | "no" | "off" | "0" => Some(false),
        // If the string doesn't match any known boolean representation
        _ => None,
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    // Attempt to read the contents of the file specified by config.file_path
    match read_file(&config.file_path) {
        Ok(contents) => {
            // If the file is successfully read, we proceed with searching based on case sensitivity
            
            // Check if the search should be case sensitive
            if config.case_sensitive {
                // Use case-sensitive search function
                for line in search_case_sensitive(&config.query, &contents) {
                    println!("{}", line);
                }
            } else {
                // Use case-insensitive search function
                for line in search_case_insensitive(&config.query, &contents) {
                    println!("{}", line);
                }
            }

            // Return Ok(()) to signify successful execution of the function
            Ok(())
        },
        Err(e) => {
            // If reading the file fails, print an error message to stderr
            eprintln!("Failed to read file: {}", e);
            
            // Instead of exiting the program, return an error. This allows the caller
            // to handle the error as needed, making the function more flexible in use.
            // Here we return a static string slice for simplicity.
            Err("Failed to read file")
        }
    }
}

pub struct Config {
    query: String,
    file_path: String,
    case_sensitive : bool,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>>{

        if args.len() < 3 {
            // eprintln!("Not enough arguments provided");
            // process::exit(1);
            return Err("Not enough arguments provided".into());
        }
    
        let query: String = args[1].to_string();
        let file_path: String = args[2].to_string();

        let case_sensitive = match args.get(3) {
            Some(arg) if arg.is_empty() => {
                eprintln!("Failed to parse case sensitivity, using default value false.");
            false
            }
            Some(arg) => match string_to_bool(arg) {
                Some(value) => value,
                None => {
                    eprintln!("Failed to parse case sensitivity, using default value.");
                    false        
                }
            },
            None => {
            eprintln!("Failed to parse case sensitivity, using default value.");
            false
            } 

        };

        Ok(Config { query, file_path, case_sensitive })
        }
}

pub fn search_case_sensitive<'a>( query : &str, content:  &'a str)-> Vec<&'a str>{
    let mut words : Vec<&str> = Vec::new();

    for word in content.lines() {
        // print!("{}", word);
        if word.contains(query) {
            words.push(&word);
        }
        else {
            continue;
        }
    }
    words

}

pub fn search_case_insensitive<'a> (query :  &str, content:  &'a str) -> Vec<&'a str>{
    let mut words : Vec<&str> = Vec::new();

    for word in content.lines() {
        // print!("{}", word);
        if word.to_lowercase().contains(&query.to_lowercase()) {
            words.push(&word);
        }
        else {
            continue;
        }
    }
    words
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "safe, fast, productive.";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}