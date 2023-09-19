use crossterm::style::Color;

pub const X_WIDTH: i16 = 2;

pub struct Point {
    pub x: i16,
    pub y: i16,
    pub color: Color,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x, y, color: Color::White, }
    }

    pub fn new_with_color(x: i16, y: i16, color: Color) -> Point {
        Point { x, y, color, }
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
