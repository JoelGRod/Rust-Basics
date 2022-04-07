use crate::basics::main::functions::infrastructure::helpers;

#[test]
fn it_adds_two() {
    // Arrange
    let digit: i32 = 2;
    // Act
    let result: i32 = helpers::add_two(digit);
    // Assert
    assert_eq!(result, 4);
}

#[test]
fn it_not_adds_three() {
    // Arrange
    let digit: i32 = 2;
    // Act
    let result: i32 = helpers::add_two(digit);
    // Assert
    assert_ne!(result, 5);
}

#[test]
fn greeting_contains_name() {
    // Arrange
    let name: &str = "Bego";
    // Act
    let result: String = helpers::greeting(name);
    // Assert
    assert!(
        result.contains(name),
        "Greeting did not contain name, value was {}",
        result
    );
}