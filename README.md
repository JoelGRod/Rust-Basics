# Rust Basic Project
* This is a basic project made with the Rust language
* Modules, structs, enums, traits
* It has been created for educational reasons

## Instructions:
* Download the code
* cargo run (at project folder):
    * cargo run [query] [file_name] -> Env variable (case sensitive)
    * CASE_INSENSITIVE=1 cargo run [query] [file_name] -> Env variable (case insensitive)
    * CASE_INSENSITIVE=1 cargo run to poem.txt
    * CASE_INSENSITIVE=1 cargo run to poem.txt > src/shared/output/stdout.txt

## Tests
* For testing execute -> cargo test

## Console App - grep
* cargo run query filename

## Macros!
* println!()    -> stdout
* eprintln!()   -> stderr
* vec![]        -> Vec::new()
* format!       -> Strings
* panic!()
[tests]
* assert!()
* assert_eq()

## Result() and Option Management
* unwrap()                      -> panic! if err
* unwrap_or_else(callback)      -> manages panic! if err
* unwrap_or()                   -> manages panic! if err
* expect()                      -> modify panic!
* is_err()                      -> true if err, false if ok