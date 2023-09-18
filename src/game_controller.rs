use std::{time::SystemTime, io::stdin};
use crate::{point::Point, board::Board, generic};

pub struct GameController {
    pub occupied: Vec<Point>,
    pub time: SystemTime,
    pub score: u32,
    pub board: Board,
}

impl GameController {
    pub fn new() -> GameController {
        generic::hide_cursor();

        // Required for running EXE directly
        println!("Press enter to start");
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();

        GameController {
            occupied: Vec::new(),
            time: SystemTime::now(),
            score: 0,
            board: Board::new(),
        }
    }

    pub fn reset(&mut self) {
        self.occupied = Vec::new();
        self.time = SystemTime::now();
        self.score = 0;

        generic::clear_terminal();
        self.board.draw_frame();
        self.board.clear_board();
        self.print_score();
    }

    /// Return if tet should autofall
    /// Reset time if true
    pub fn should_autofall(&mut self) -> bool {
        let should_fall = self.get_time_elapsed() >= 1000;
        if should_fall {
            self.time = SystemTime::now();
        }

        should_fall
    }

    pub fn place_tet(&mut self) {
        let rows = self.clear_full_rows();
        self.update_score(rows as u32);
    }

    pub fn update_score(&mut self, cleared_rows: u32) {
        self.score += 3_u32.pow(cleared_rows + 1);
        self.print_score();
    }

    /// Clear rows that span entire width of board
    ///
    /// Returns cleared row count
    pub fn clear_full_rows(&mut self) -> u8 {
        let rows = self.get_row_count();

        // Which rows should move down, and how far
        let mut move_down_arr: Vec<u8> = vec![0; self.board.get_height() as usize];
        let mut cleared_rows: u8 = 0;
        for i in (0..move_down_arr.len()).rev() {
            if rows[i] == self.board.get_width() {
                cleared_rows += 1;
            } else {
                move_down_arr[i] = cleared_rows;
            }
        }

        // Nothing was moved down, so no rows were cleared
        if cleared_rows == 0 {
            return 0;
        }

        // Move rows down and save rows to remove
        let mut indexes_to_remove: Vec<usize> = Vec::new();
        for i in 0..self.occupied.len() {
            // NOTE: Panics if Y is negative (which it is when Tet spawns)
            if rows[self.occupied[i].y as usize] == self.board.get_width() {
                indexes_to_remove.push(i);
            } else {
                self.occupied[i].y += move_down_arr[self.occupied[i].y as usize] as i16;
            }
        }

        // Remove rows
        for i in (indexes_to_remove).iter().rev() {
            self.occupied.remove(*i);
        }

        // Print updates
        self.print_occupied();

        cleared_rows
    }

    pub fn collision_check(&self, points: [Point; 4], x: i16, y: i16) -> bool {
        for i in 0..=3 {
            let point = points[i];
            if point.y + y == self.board.get_height() as i16 {
                return true;
            }

            if point.x + x == self.board.get_width() as i16 || point.x + x == -1 {
                return true;
            }

            for occ in &self.occupied {
                if point.x + x == occ.x && point.y + y == occ.y {
                    return true;
                }
            }
        }

        return false;
    }

    /// Get all rows with count of occupied spaces
    fn get_row_count(&self) -> Vec<u16> {
        let mut rows: Vec<u16> = vec![0; self.board.get_height() as usize];

        for occ in &self.occupied {
            // NOTE: Panics if Y is negative (which it is when Tet spawns)
            rows[occ.y as usize] += 1;
        }

        rows
    }

    /// Clear board and print occupied points
    fn print_occupied(&self) {
        self.board.clear_board();

        for occ in &self.occupied {
            generic::move_cursor(occ.x_width() as u16 + self.board.get_offset_x(), occ.y as u16 + self.board.get_offset_y());
            print!("[]");
        }

        generic::move_cursor(0, 0);
    }

    fn print_score(&self) {
        generic::move_cursor(28, 2);
        println!("Score: {:?}", self.score);
    }

    fn get_time_elapsed(&self) -> u128 {
        self.time.elapsed().unwrap().as_millis()
    }
}