use std::io::stdout;
use crossterm::{cursor, terminal, Command, ExecutableCommand};
use crate::point::Point;

pub const H:u16 = 20;
pub const W:u16 = 10;

pub fn term_command(command: impl Command) {
    stdout().execute(command).unwrap();
}

pub fn move_cursor(x: u16, y: u16) {
    term_command(cursor::MoveTo(x, y));
}

#[allow(dead_code)]
pub fn debug_print(y: u16, print: &str) {
    move_cursor(15, y);
    print!("{}", print);
}

pub fn collision_check(points: [Point; 4], occupied: &Vec<Point>, x: i16, y: i16) -> bool {
    for i in 0..=3 {
        let point = points[i];
        if point.y + y == self::H as i16 {
            return true;
        }

        if point.x + x == self::W as i16 || point.x + x == -1 {
            return true;
        }

        for occ in occupied {
            if point.x + x == occ.x && point.y + y == occ.y {
                return true;
            }
        }
    }

    return false;
}

pub fn clear_terminal() {
    term_command(terminal::Clear(terminal::ClearType::All));
}

/// Clear board with default background
///
/// Call `clear_terminal()` to clear entire terminal
pub fn clear_board() {
    move_cursor(0, 0);

    for y in 0..H {
        for x in 0..W {
            move_cursor(x, y);
            print!(".");
        }
    }

    move_cursor(0, 0);
}

pub fn hide_cursor() {
    term_command(cursor::Hide);
}
