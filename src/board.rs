use crate::{generic, point};

pub struct Board {
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16,
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 10,
            height: 20,
            offset_x: 5,
            offset_y: 2
        }
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_offset_x(&self) -> u16 {
        self.offset_x
    }

    pub fn get_offset_y(&self) -> u16 {
        self.offset_y
    }

    pub fn draw_frame(&self) {
        generic::move_cursor(0, 0);

        for y in 0..self.height {
            generic::move_cursor(self.offset_x - 2, y + self.offset_y);
            print!("<|");

            generic::move_cursor(self.offset_x + self.width * point::X_WIDTH as u16, y + self.offset_y);
            print!("|>");
        }

        for x in 0..self.width * point::X_WIDTH as u16 {
            generic::move_cursor(x + self.offset_x, self.offset_y - 1);
            print!("_");

            generic::move_cursor(x + self.offset_x, self.height + self.offset_y);
            print!("‾");
        }

        generic::move_cursor(0, 0);
    }

    pub fn clear_board(&self) {
        generic::move_cursor(0, 0);

        for y in self.offset_y..self.height + self.offset_y {
            for x in self.offset_x..self.width * point::X_WIDTH as u16 + self.offset_x {
                generic::move_cursor(x, y);
                print!(" ");
            }
        }

        generic::move_cursor(0, 0);
    }
}
