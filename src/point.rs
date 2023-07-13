pub const X_WIDTH: i16 = 2;

pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x, y, }
    }

    /// x multiplied by X_WIDTH
    pub fn x_width(&self) -> i16 {
        self.x * self::X_WIDTH
    }
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
