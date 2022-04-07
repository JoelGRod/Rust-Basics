
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