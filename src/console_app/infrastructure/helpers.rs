
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        // Arrange
        let query: &str = "example";
        let content: &str = "\
            Hello Rust
            this is a example TDD test
            and nothing else
            Example tape
        ";
        // Act
        // Assert
        assert_eq!(
            vec!["this is a example TDD test"], 
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        // Arrange
        let query: &str = "rUsT";
        let content: &str = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.
        ";
        // Act
        // Assert
        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, content)
        );
    }
}