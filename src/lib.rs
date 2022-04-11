// Lib Modules
mod basics;
mod exercises;
mod console_app;

// Imports
use crate::basics::{
    structs::app::lights_main as structs, 
    collections::app::collections_main as collections,
    errors::app::errors_main as errors,
    traits_generics::app::traits_main as traits
};
use std::{env, fs::File, io::ErrorKind};

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
pub fn console_app() {
    let args: Vec<String> = env::args().collect();
    
    let query: &str = args.get(1).unwrap_or_else(|| { "test" });
    let filename: &str = args.get(2).unwrap_or_else(|| { "hello.txt" });

    println!("Query: {}", query);
    println!("Filename: {}", filename);
}

// Advanced
// Exercises