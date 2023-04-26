use rand::Rng;

use crate::generic;
use crate::point::Point;

pub struct Tet {
    pub pos: Point,
    pub pivot: Point,
    pub model: [Point; 4],
}

static DEFAULT_POS: Point = Point {x: 5, y: 0};

impl Tet {
    fn new_i() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
            ],
        }
    }

    fn new_l() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(1, 2),
            ],
        }
    }

    fn new_j() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(0, 1),
            model: [
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(-1, 2),
            ],
        }
    }

    fn new_t() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(1, 0),
            model: [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(2, 0),
            ],
        }
    }

    fn new_o() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(0, 0),
            model: [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
        }
    }

    fn new_s() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(1, 0),
            model: [
                Point::new(2, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(0, 0),
            ],
        }
    }

    fn new_z() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: Point::new(1, 0),
            model: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
        }
    }

    pub fn new_random() -> Tet {
        let rnd = rand::thread_rng().gen_range(0..=5);
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
            Point::new(
                self.pos.x - self.pivot.x + self.model[0].x,
                self.pos.y - self.pivot.y + self.model[0].y
            ),
            Point::new(
                self.pos.x - self.pivot.x + self.model[1].x,
                self.pos.y - self.pivot.y + self.model[1].y,
            ),
            Point::new(
                self.pos.x - self.pivot.x + self.model[2].x,
                self.pos.y - self.pivot.y + self.model[2].y,
            ),
            Point::new(
                self.pos.x - self.pivot.x + self.model[3].x,
                self.pos.y - self.pivot.y + self.model[3].y,
            ),
        ];
    }

    pub fn place(&self, occupied: &mut Vec<Point>) {
        let points = self.points_pos();
        occupied.append(&mut points.to_vec());
    }

    pub fn print(&self, remove: bool) {
        let points = self.points_pos();
        for i in 0..=3 {
            let y: i16 = points[i].y;
            if y < 0 {
                continue;
            }

            generic::move_cursor(
                points[i].x as u16,
                y as u16
            );

            if remove {
                print!(".");
            } else {
                print!("■");
            }

            generic::move_cursor(0, 0);
        }
    }

    pub fn translate(&mut self, x: i16, y: i16) {
        self.print(true);
        self.pos.x += x;
        self.pos.y += y;
        self.print(false);
    }

    pub fn rotate(&mut self, occupied: &Vec<Point>) {
        // TODO: Fix weird flipping on i, z, s, o
        self.print(true);
        for i in 0..=3 {
            let x = self.model[i].x;
            let y = self.model[i].y;

            self.model[i].x = y;
            self.model[i].y = -x;
        }

        let x = self.pivot.x;
        let y = self.pivot.y;
        self.pivot.x = y;
        self.pivot.y = -x;

        // TODO: Try to help the player instead of just disallowing rotation
        if self.collision_check(&occupied, 0, 0) {
            for i in 0..=3 {
                let x = self.model[i].x;
                let y = self.model[i].y;

                self.model[i].x = -y;
                self.model[i].y = x;
            }

            let x = self.pivot.x;
            let y = self.pivot.y;
            self.pivot.x = -y;
            self.pivot.y = x;
        }

        self.print(false);
    }

    pub fn collision_check(&self, occupied: &Vec<Point>, x: i16, y: i16) -> bool {
        let points = self.points_pos();
        for i in 0..=3 {
            let point = points[i];
            if point.y + y == generic::H as i16 {
                return true;
            }

            if point.x + x == generic::W as i16 + 1 || point.x + x == -1 {
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
}


