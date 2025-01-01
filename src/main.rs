use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: &str) -> String {

    let mut file = File::open(filename).expect("file is not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error with content of the file");
    contents

}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {query} and in file {file_path}");

    let contents = read_file("poem.txt");

    println!("the file has following contents {contents}");

    Ok(())

}
