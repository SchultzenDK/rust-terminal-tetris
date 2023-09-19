use crossterm::style::Color;
use rand::Rng;
use crate::{point::Point, generic, game_controller::GameController, board::Board};

pub struct Tet {
    pub pos: Point,
    pivot: Point,
    model: [Point; 4],
    color: Color,
    allowed_flips: u8,
    flips: u8,
    rotate_clockwise: bool,
}

impl Tet {
    fn new_i() -> Tet {
        Tet {
            pos: Point::new(4, -3),
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
            ],
            color: Color::Blue,
            allowed_flips: 1,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    fn new_l() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(1, 2),
            ],
            color: Color::Red,
            allowed_flips: 3,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    fn new_j() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(-1, 2),
            ],
            color: Color::Green,
            allowed_flips: 3,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    fn new_t() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(1, 0),
            model: [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(2, 0),
            ],
            color: Color::Yellow,
            allowed_flips: 3,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    fn new_o() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(0, 0),
            model: [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
            color: Color::Magenta,
            allowed_flips: 0,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    fn new_s() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(1, 1),
            model: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
            color: Color::Cyan,
            allowed_flips: 1,
            flips: 0,
            rotate_clockwise: false,
        }
    }

    fn new_z() -> Tet {
        Tet {
            pos: Point::new(4, -2),
            pivot: Point::new(1, 1),
            model: [
                Point::new(2, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(0, 0),
            ],
            color: Color::DarkYellow,
            allowed_flips: 1,
            flips: 0,
            rotate_clockwise: true,
        }
    }

    pub fn new_random() -> Tet {
        let rnd: u8 = rand::thread_rng().gen_range(0..=6);
        match rnd {
            0 => return Tet::new_i(),
            1 => return Tet::new_l(),
            2 => return Tet::new_j(),
            3 => return Tet::new_t(),
            4 => return Tet::new_o(),
            5 => return Tet::new_s(),
            _ => return Tet::new_z(),
        }
    }

    /// Get board position of individual points in model
    pub fn points_pos(&self) -> [Point; 4] {
        return [
            Point::new_with_color(
                self.pos.x - self.pivot.x + self.model[0].x,
                self.pos.y - self.pivot.y + self.model[0].y,
                self.color,
            ),
            Point::new_with_color(
                self.pos.x - self.pivot.x + self.model[1].x,
                self.pos.y - self.pivot.y + self.model[1].y,
                self.color,
            ),
            Point::new_with_color(
                self.pos.x - self.pivot.x + self.model[2].x,
                self.pos.y - self.pivot.y + self.model[2].y,
                self.color,
            ),
            Point::new_with_color(
                self.pos.x - self.pivot.x + self.model[3].x,
                self.pos.y - self.pivot.y + self.model[3].y,
                self.color,
            ),
        ];
    }

    pub fn can_place(&self) -> bool {
        for point in self.points_pos() {
            if point.y < 0 {
                return false;
            }
        }

        return true;
    }

    /// Place Tet, return success
    pub fn place(&self, occupied: &mut Vec<Point>) -> bool {
        if !self.can_place() {
            return false;
        }

        let points = self.points_pos();
        occupied.append(&mut points.to_vec());

        return true;
    }

    pub fn print(&self, remove: bool, board: &Board) {
        generic::set_color(self.color);

        let points = self.points_pos();
        for i in 0..=3 {
            let y: i16 = points[i].y;
            if y < 0 {
                continue;
            }

            generic::move_cursor(
                points[i].x_width() as u16 + board.get_offset_x(),
                y as u16 + board.get_offset_y()
            );

            if remove {
                print!("  ");
            } else {
                print!("[]");
            }

            generic::move_cursor(0, 0);
        }

        generic::set_color(Color::Reset);
    }

    /// Translate if there's no collision
    ///
    /// Returns true on success or false if unable to move
    pub fn translate(&mut self, x: i16, y: i16, game_controller: &GameController) -> bool {
        if game_controller.collision_check(self.points_pos(), x, y) {
            return false;
        }

        self.print(true, &game_controller.board);
        self.pos.x += x;
        self.pos.y += y;
        self.print(false, &game_controller.board);

        return true;
    }

    pub fn rotate(&mut self, game_controller: &GameController) {
        if self.allowed_flips == 0 {
            return;
        }

        let mut clone = self.clone();

        let reset_flip = clone.allowed_flips == clone.flips;

        if reset_flip {
            for _ in 0..clone.flips {
                clone.rotate_model(clone.rotate_clockwise);
            }
        } else {
            clone.rotate_model(!clone.rotate_clockwise);
        }

        // Help player by getting closest free position
        let mut success = false;
        for y in 0..=2 {
            for x in 0..=2 {
                if !game_controller.collision_check(clone.points_pos(), x, -y) {
                    clone.pos.x += x;
                    success = true;
                    break;
                }

                if !game_controller.collision_check(clone.points_pos(), -x, -y) {
                    clone.pos.x -= x;
                    success = true;
                    break;
                }
            }

            if success {
                clone.pos.y -= y;
                break;
            }
        }

        if !success {
            // Could not rotate
            return;
        }

        // Update and print `self`
        self.print(true, &game_controller.board);
        if !reset_flip {
            self.flips += 1;
        } else {
            self.flips = 0;
        }
        self.model = clone.model;
        self.pivot = clone.pivot;
        self.pos = clone.pos;
        self.print(false, &game_controller.board);
    }

    /// Move tet down and place if able
    ///
    /// If unable to place or move, return false, otherwise return true
    pub fn move_down(&mut self, game_controller: &mut GameController) -> bool {
        if !self.translate(0, 1, &game_controller) {
            if !self.place(&mut game_controller.occupied) {
                return false;
            }

            game_controller.place_tet();

            *self = Tet::new_random();
        }

        return true;
    }

    fn rotate_model(&mut self, clockwise: bool) {
        for i in 0..=3 {
            let x: i16 = self.model[i].x;
            let y = self.model[i].y;

            if clockwise {
                self.model[i].x = y;
                self.model[i].y = -x;
            } else {
                self.model[i].x = -y;
                self.model[i].y = x;
            }
        }

        // Rotate pivot
        let x = self.pivot.x;
        let y = self.pivot.y;

        if clockwise {
            self.pivot.x = y;
            self.pivot.y = -x;
        } else {
            self.pivot.x = -y;
            self.pivot.y = x;
        }
    }
}

impl Copy for Tet {}

impl Clone for Tet {
    fn clone(&self) -> Self {
        *self
    }
}
