
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        // Arrange
        let query: &str = "example";
        let content: &str = "\
            Hello Rust
            this is a example TDD test
            and nothing else
        ";
        // Act
        // Assert
        assert_eq!(vec!["this is a example TDD test"], search(query, content));
    }
}