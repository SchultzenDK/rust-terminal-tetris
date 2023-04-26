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

    pub fn points_pos(&self) -> [i16; 2] {
        return [self.pos[0] - self.pivot[0], self.pos[1] - self.pivot[1]];
    }

    pub fn place(&self, occupied: &mut Vec<[i16; 2]>) {
        let pos = self.points_pos();
        for i in 0..=3 {
            occupied.push([pos[0] + self.model[i][0], pos[1] + self.model[i][1]]);
        }
    }

    pub fn print(&self, remove: bool) {
        let pos = self.points_pos();
        for i in 0..=3 {
            let y: i16 = pos[1] + self.model[i][1];
            if y < 0 {
                continue;
            }

            generic::move_cursor(
                (pos[0] + self.model[i][0]) as u16,
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

    pub fn collision_check(&self, occupied: &Vec<[i16; 2]>, x: i16, y: i16) -> bool {
        let mut pos = self.points_pos();
        pos[0] += x;
        pos[1] += y;
        for i in 0..=3 {
            let point = self.model[i];
            if point[1] + pos[1] == generic::H as i16 {
                return true;
            }

            if point[0] + pos[0] == generic::W as i16 + 1{
                return true;
            } else if point[0] + pos[0] == -1 {
                return true;
            }

            for occ in occupied {
                if point[0] + pos[0] == occ[0] && point[1] + pos[1] == occ[1]{
                    return true;
                }
            }
        }

        return false;
    }
}


