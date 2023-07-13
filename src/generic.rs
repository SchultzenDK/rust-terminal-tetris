use std::io::stdout;
use crossterm::{cursor, terminal, Command, ExecutableCommand};
use crate::{point::Point, board::Board};

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
    move_cursor(0, 0);
}

pub fn collision_check(points: [Point; 4], occupied: &Vec<Point>, x: i16, y: i16, board: &Board) -> bool {
    for i in 0..=3 {
        let point = points[i];
        if point.y + y == board.get_height() as i16 {
            return true;
        }

        if point.x + x == board.get_width() as i16 || point.x + x == -1 {
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

pub fn hide_cursor() {
    term_command(cursor::Hide);
}
