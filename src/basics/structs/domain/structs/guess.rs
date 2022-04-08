pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.", 
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.", 
                value
            )
        }
        Self { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1, got 0.")]
    fn should_panic_with_lower_than_one() {
        // Arrange
        Guess::new(0);
        // Act
        // Assert
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 101.")]
    fn should_panic_with_greater_than_one_hundred() {
        // Arrange
        Guess::new(101);
        // Act
        // Assert
    }    
}