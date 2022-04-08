use std::{fs::File, io::ErrorKind};

use std::io::{self, Read};
use std::fs;

pub fn errors_examples() {
    /* ---------- Unrecoverable problems -> panic! ---------- */
    // panic!("This is an unrecoverable error");

    // RUST_BACKTRACE=1 cargo run -> Execute program with backtrace

    /* ---------- Recoverable problems -> result<T, E> ---------- */
    /*
        How do we know File::open returns a Result? We could look at 
        the standard library API documentation, or we could ask the 
        compiler! If we give f a type annotation that we know is not 
        the return type of the function and then try to compile the 
        code, the compiler will tell us that the types don’t match. 
        The error message will then tell us what the type of f is.
     */
    let f: Result<File, std::io::Error> = File::open("src/output/hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("src/output/hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Error creating the file {:?}", error)
    //         },
    //         other => panic!("Error opening the file {:?}", other)
    //     }
    // };

    // Alternatives to match for handling errors

    // unwrap_or_else(error) -> Cleaner
    let f = File::open("src/output/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("src/output/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap() -> Returns ok or panic!
    // let f = File::open("src/output/hello.txt").unwrap();

    // expect("some error") -> Same as above but with custom error message
    // let f = File::open("src/output/hello.txt").expect("Problem opening the file");
}


// Propagating errors
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// Same as above using ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// Same as above even shorter
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// Same as above even shorter
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("src/output/hello.txt")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_result_ok() -> Result<(), io::Error> {
        read_username_from_file()?;
        Ok(())
    }
}