use std::io::stdout;
use crossterm::{cursor::MoveTo, ExecutableCommand};

pub static H:u16 = 20;
pub static W:u16 = 10;

pub fn move_cursor(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).unwrap();
}

#[allow(dead_code)]
pub fn debug_print(y: u16, print: &str) {
    move_cursor(30, y);
    print!("{}", print);
}
