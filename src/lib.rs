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
use std::{env, fs};

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
pub fn console_app(query: &str, filename: &str) {
    let path: String = "src/shared/files/".to_owned();

    println!("Query: {}", query);
    println!("Filename: {}", filename);

    let contents: String = fs::read_to_string(path + filename)
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}

// Advanced
// Exercises