use std::fmt::Display;

// Example Struct I
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }

    // The point that calls this loses the ownership
    pub fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point { 
            x: self.x, 
            y: other.y, 
        }
    }
}

// Limited implementation based on the types of generics
impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Example Struct II
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Limited implementation based on the traits the generics implements
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}