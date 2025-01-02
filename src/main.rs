use std::env;
use std::error::Error;
use std::process;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);


    // let (query, file_path) = parse_config(&args);
    
    let config: Config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);

    Ok(())
}

fn run(config : Config) -> Result<(), &'static str>{
    match lib::read_file(&config.file_path) {
        Ok(contents) => {
            println!("The file has the following contents:\n{contents}");
        Ok(())},
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            process::exit(1);
        }
    }
}

struct Config {
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

// fn parse_config(args: &[String]) -> Config {
//     let query: String = args[1].to_string();
//     let file_path: String = args[2].to_string();

//     Config { query, file_path }
// }
