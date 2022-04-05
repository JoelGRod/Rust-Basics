pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl ImportantExcerpt<'_> {
    pub fn print_part(&self) {
        println!("{}", self.part);
    }
}