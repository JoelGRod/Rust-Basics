use std::{env, process};

// Entry Point
/*
    main function responsibilities:
        Calling the command line parsing logic with the argument values
        Setting up any other configuration
        Calling a run function in lib.rs
        Handling the error if run returns an error
*/
fn main() {
    // Basics
    // basics_rust::collections_example();
    // basics_rust::errors_example();
    // basics_rust::structs_example();
    // basics_rust::traits_example();

    // minigrep
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    basics_rust::console_app(&config.query, &config.filename);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Self {query, filename})
    }
}
