use crate::basics::main::structs::domain::structs::guess::Guess;

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