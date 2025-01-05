use std::env;
use std::error::Error;
use std::process;

mod lib;
use crate::lib::Config;
use crate::lib::run;



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    
    let config: Config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let _ = run(config)?;

    // another way to handle error 
    //     if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    Ok(())
}