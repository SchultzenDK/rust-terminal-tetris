use std::io::stdout;
use crossterm::{cursor::MoveTo, ExecutableCommand};
use crate::point::Point;

pub const H:u16 = 20;
pub const W:u16 = 10;

pub fn move_cursor(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).unwrap();
}

#[allow(dead_code)]
pub fn debug_print(y: u16, print: &str) {
    move_cursor(15, y);
    print!("{}", print);
}

pub fn collision_check(points: [Point; 4], occupied: &Vec<Point>, x: i16, y: i16) -> bool {
    for i in 0..=3 {
        let point = points[i];
        // FIXME: Tet stops one y too early if running exe directly (outside of terminal)
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
