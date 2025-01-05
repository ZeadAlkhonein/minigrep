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



pub fn run(config: Config) -> Result<(), &'static str> {
    // Attempt to read the contents of the file specified by config.file_path
    match read_file(&config.file_path) {
        Ok(contents) => {
            // If the file is successfully read, print its contents to stdout
            // println!("The file has the following contents:\n{contents}");
            // Return Ok(()) to signify successful execution of the function
            // let vec = search(&config.query, &contents);

            // if vec.is_empty() {
            //     println!("The vector is empty.");
            // } else {
            //     println!("The vector is not empty.");
            // }

            for line in search(&config.query, &contents) {
                println!("{line}");
            }

            Ok(())
        },
        Err(e) => {
            // If reading the file fails, print an error message to stderr
            eprintln!("Failed to read file: {}", e);
            // Exit the program with a failure status code
            // Note: Using `process::exit(1)` here will terminate the program immediately, 
            // which might not be ideal in all contexts (e.g., if this function was part of a larger system).
            // An alternative could be to return `Err("Failed to read file")` or similar.
            process::exit(1);
        }
    }
}

pub struct Config {
    query: String,
    file_path: String,

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
        Ok(Config { query, file_path })
        }
}

pub fn search<'a>( query : &str, content:  &'a str)-> Vec<&'a str>{
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe, fast, productive.";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}