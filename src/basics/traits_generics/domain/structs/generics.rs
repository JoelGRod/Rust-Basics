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

impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}