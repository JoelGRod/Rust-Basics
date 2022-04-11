use std::env;

// Entry Point
fn main() {
    // Basics
    // basics_rust::collections_example();
    // basics_rust::errors_example();
    // basics_rust::structs_example();
    // basics_rust::traits_example();

    // minigrep
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
