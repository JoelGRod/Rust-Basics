pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Self {query, filename})
    }
}