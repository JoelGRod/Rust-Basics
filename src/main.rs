use std::{env, process};
use basics_rust::console_app::domain::config::Config;

/*
    Entry Point -> main()
    main function responsibilities:
        * Calling the command line parsing logic with the argument values.
        * Setting up any other configuration.
        * Handling errors from the configuration process.
        * Calling a run function in lib.rs.
        * Handling the error if run returns an error.
        * * We’ll extract a function named run (in lib.rs) that will hold 
        all the logic currently in the main function that isn’t involved 
        with setting up configuration or handling errors.
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
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = basics_rust::run_console_app(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    };
}
