pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Self { part }
    }

    pub fn print_part(&self) {
        println!("{}", self.part);
    }
}