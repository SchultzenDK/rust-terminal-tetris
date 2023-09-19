use std::time::SystemTime;
use crossterm::{event::KeyCode, style::Color};
use crate::{point::Point, board::Board, generic, input_controller::InputController};

const LEVEL_SCALE: u8 = 5;
const LEVEL_AT_SCORE: u16 = 150;
const INITIAL_FALL_MS: u16 = 750;

pub struct GameController {
    pub occupied: Vec<Point>,
    time: SystemTime,
    score: u32,
    pub board: Board,
    level: u32,
    fall_ms: u16,
}

impl GameController {
    pub fn new() -> GameController {
        generic::clear_terminal();

        let this = GameController {
            occupied: Vec::new(),
            time: SystemTime::now(),
            score: 0,
            board: Board::new(),
            level: 1,
            fall_ms: INITIAL_FALL_MS,
        };

        this.board.draw_frame();
        this.board.clear_board();
        this.print_score();
        this.print_level();

        this
    }

    pub fn reset_time(&mut self) {
        self.time = SystemTime::now();
    }

    /// Return if tet should autofall
    /// Reset time if true
    pub fn should_autofall(&mut self) -> bool {
        let should_fall = self.get_time_elapsed() >= self.fall_ms as u128;
        if should_fall {
            self.time = SystemTime::now();
        }

        should_fall
    }

    pub fn place_tet(&mut self) {
        let rows = self.clear_full_rows();
        self.update_score(rows as u32);
        self.update_level();
    }

    fn set_fall_ms(&mut self) {
        if self.level == 1 {
            return;
        }

        let level = self.level - 1;
        self.fall_ms = (INITIAL_FALL_MS as f32 / (1_f32 + level as f32 / LEVEL_SCALE as f32)).floor() as u16;
    }

    fn update_score(&mut self, cleared_rows: u32) {
        self.score += 3_u32.pow(cleared_rows + 1);
        self.print_score();
    }

    fn update_level(&mut self) {
        self.level = (self.score as f32 / LEVEL_AT_SCORE as f32).ceil() as u32;
        self.print_level();
        self.set_fall_ms();
    }

    /// Clear rows that span entire width of board
    ///
    /// Returns cleared row count
    fn clear_full_rows(&mut self) -> u8 {
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

    /// Display game over message and restart or quit options
    ///
    /// Returns true on quit or false on restart
    pub fn game_over(input_controller: &mut InputController) -> bool {
        // Game over
        generic::move_cursor(28, 10);
        println!("Game over");
        generic::move_cursor(28, 11);
        println!("Press ENTER to try again,");
        generic::move_cursor(28, 12);
        println!("or ESC to quit");

        loop {
            input_controller.update();
            if input_controller.key_pressed(KeyCode::Enter) {
                return false;
            } else if input_controller.key_pressed(KeyCode::Esc) {
                generic::clear_terminal();
                generic::move_cursor(0, 0);
                generic::hide_cursor(false);
                return true;
            }
        }
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
            generic::set_color(occ.color);
            generic::move_cursor(occ.x_width() as u16 + self.board.get_offset_x(), occ.y as u16 + self.board.get_offset_y());
            print!("[]");
        }

        generic::set_color(Color::Reset);
        generic::move_cursor(0, 0);
    }

    fn print_score(&self) {
        generic::move_cursor(28, 2);
        println!("Score: {:?}", self.score);
    }

    fn print_level(&self) {
        generic::move_cursor(28, 4);
        println!("Level: {:?}", self.level);
    }

    fn get_time_elapsed(&self) -> u128 {
        self.time.elapsed().unwrap().as_millis()
    }
}