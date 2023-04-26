use rand::Rng;

use crate::generic;

pub struct Tet {
    pub pos: [i16; 2],
    pub pivot: [i16; 2],
    pub model: [[i16; 2]; 4],
}

static DEFAULT_POS: [i16; 2] = [5, 0];

impl Tet {
    fn new_i() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                0, 1
            ],
            model: [
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3]
            ],
        }
    }

    fn new_l() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                0, 1
            ],
            model: [
                [0, 0],
                [0, 1],
                [0, 2],
                [1, 2]
            ],
        }
    }

    fn new_j() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                0, 1
            ],
            model: [
                [0, 0],
                [0, 1],
                [0, 2],
                [-1, 2],
            ],
        }
    }

    fn new_t() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                1, 0
            ],
            model: [
                [0, 0],
                [1, 0],
                [1, 1],
                [2, 0],
            ],
        }
    }

    fn new_o() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                0, 0
            ],
            model: [
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
            ],
        }
    }

    fn new_s() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                1, 0
            ],
            model: [
                [2, 1],
                [1, 1],
                [1, 0],
                [0, 0],
            ],
        }
    }

    fn new_z() -> Tet {
        Tet {
            pos: self::DEFAULT_POS,
            pivot: [
                1, 0
            ],
            model: [
                [0, 1],
                [1, 1],
                [1, 0],
                [2, 0],
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
    pub fn points_pos(&self) -> [[i16; 2]; 4] {
        return [
            [
                self.pos[0] - self.pivot[0] + self.model[0][0],
                self.pos[1] - self.pivot[1] + self.model[0][1],
            ],
            [
                self.pos[0] - self.pivot[0] + self.model[1][0],
                self.pos[1] - self.pivot[1] + self.model[1][1],
            ],
            [
                self.pos[0] - self.pivot[0] + self.model[2][0],
                self.pos[1] - self.pivot[1] + self.model[2][1],
            ],
            [
                self.pos[0] - self.pivot[0] + self.model[3][0],
                self.pos[1] - self.pivot[1] + self.model[3][1],
            ],
        ];
    }

    pub fn place(&self, occupied: &mut Vec<[i16; 2]>) {
        let points = self.points_pos();
        for i in 0..=3 {
            occupied.push([points[i][0], points[i][1]]);
        }
    }

    pub fn print(&self, remove: bool) {
        let points = self.points_pos();
        for i in 0..=3 {
            let y: i16 = points[i][1];
            if y < 0 {
                continue;
            }

            generic::move_cursor(
                (points[i][0]) as u16,
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
        self.pos[0] += x;
        self.pos[1] += y;
        self.print(false);
    }

    pub fn rotate(&mut self, occupied: &Vec<[i16; 2]>) {
        // TODO: Fix weird flipping on i, z, s, o
        self.print(true);
        for i in 0..=3 {
            let x = self.model[i][0];
            let y = self.model[i][1];

            self.model[i][0] = y;
            self.model[i][1] = -x;
        }

        let x = self.pivot[0];
        let y = self.pivot[1];
        self.pivot[0] = y;
        self.pivot[1] = -x;

        // TODO: Try to help the player instead of just disallowing rotation
        if self.collision_check(&occupied, 0, 0) {
            for i in 0..=3 {
                let x = self.model[i][0];
                let y = self.model[i][1];

                self.model[i][0] = -y;
                self.model[i][1] = x;
            }

            let x = self.pivot[0];
            let y = self.pivot[1];
            self.pivot[0] = -y;
            self.pivot[1] = x;
        }

        self.print(false);
    }

    pub fn collision_check(&self, occupied: &Vec<[i16; 2]>, x: i16, y: i16) -> bool {
        let points = self.points_pos();
        for i in 0..=3 {
            let point = points[i];
            if point[1] + y == generic::H as i16 {
                return true;
            }

            if point[0] + x == generic::W as i16 + 1 || point[0] + x == -1 {
                return true;
            }

            for occ in occupied {
                if point[0] + x == occ[0] && point[1] + y == occ[1] {
                    return true;
                }
            }
        }

        return false;
    }
}


