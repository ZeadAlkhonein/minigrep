// src/lib.rs

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

