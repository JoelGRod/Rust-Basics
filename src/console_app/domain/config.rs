use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let filename: String = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query, 
            filename,
            case_sensitive
        })
    }
}