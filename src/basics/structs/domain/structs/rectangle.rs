pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height:i32) -> Self {
        Self { width, height }
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.height && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // Arrange
        let larger: Rectangle = Rectangle::new(8, 7);
        let smaller: Rectangle = Rectangle::new(5, 3);
        // Act
        let result: bool = larger.can_hold(&smaller);
        // Assert
        assert!(result);
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        // Arrange
        let larger: Rectangle = Rectangle::new(8, 7);
        let smaller: Rectangle = Rectangle::new(5, 3);
        // Act
        let result: bool = smaller.can_hold(&larger);
        // Assert
        assert!(!result);
    }
}