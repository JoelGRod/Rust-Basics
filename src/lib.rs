// Lib Modules
mod basics;
mod exercises;
pub mod console_app;
mod advanced;


// Imports: Basics Domain
use crate::basics::{
    structs::app::lights_main as structs, 
    collections::app::collections_main as collections,
    errors::app::errors_main as errors,
    traits_generics::app::traits_main as traits
};
// Imports: Console App
use std::{
    fs, 
    error::Error
};
use crate::console_app::{
    domain::config::Config,
    infrastructure::helpers
};

// Basics
pub fn structs_example() -> bool {
    structs::lights();
    true
}

pub fn collections_example() -> bool {
    collections::collections();
    true
}

pub fn errors_example() -> bool {
    errors::errors_examples();
    true
}

pub fn traits_example() -> bool {
    traits::traits_examples();
    true
}

// Console App - Grep
pub fn run_console_app(config: Config) -> Result<(), Box<dyn Error>> {
    let path: String = "src/shared/files/".to_owned();

    // println!("Query: {}", config.query);
    // println!("Filename: {}", config.filename);

    let contents: String = fs::read_to_string(path + &config.filename)?;

    // println!("With Text: {}", contents);
    let results: Vec<&str> = if config.case_sensitive {
        helpers::search(&config.query, &contents)
    } else {
        helpers::search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

// Advanced
// Exercises