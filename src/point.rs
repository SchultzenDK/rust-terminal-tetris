pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x, y, }
    }
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
