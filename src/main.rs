use std::{time::{SystemTime, Duration}, io::stdout};
use crossterm::{event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind}, cursor::{MoveTo, Hide}, ExecutableCommand, terminal};
use rand::Rng;

struct Tet {
    pos: [i16; 2],
    pivot: [i16; 2],
    model: [[i16; 2]; 4],
}

impl Tet {
    fn new_i() -> Tet {
        Tet {
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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
            pos: [
                5, 0
            ],
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

    fn new_random() -> Tet {
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
}

static H:u16 = 20;
static W:u16 = 10;

fn main() {
    setup();

    let mut occupied: Vec<[i16; 2]> = Vec::new();
    let mut tet = Tet::new_random();
    let mut time = SystemTime::now();

    loop {
        // Auto fall
        if time.elapsed().unwrap().as_secs() >= 1 {
            if collision_check(&tet, &occupied, 0, 1) {
                place_tet(&tet, &mut occupied);
                tet = Tet::new_random();
            }

            move_tet(&mut tet, 0, 1);
            time = SystemTime::now();
        }

        // Controls
        if poll(Duration::from_secs(0)).unwrap() {
            let event = crossterm::event::read().unwrap();

            // Move left
            if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, &occupied, -1, 0) {
                    move_tet(&mut tet, -1, 0);
                }
            // Move right
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, &occupied, 1, 0) {
                    move_tet(&mut tet, 1, 0);
                }
            // Move down
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, &occupied, 0, 1) {
                    move_tet(&mut tet, 0, 1);
                }
                if collision_check(&tet, &occupied, 0, 1) {
                    place_tet(&tet, &mut occupied);
                    tet = Tet::new_random();
                }
                time = SystemTime::now();
            // Rotate left
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::NONE, KeyEventKind::Press)) {
                // TODO: Fix weird flipping on i, z, s, o
                print_tet(&mut tet, true);
                for i in 0..=3 {
                    let x = tet.model[i][0];
                    let y = tet.model[i][1];

                    tet.model[i][0] = y;
                    tet.model[i][1] = -x;
                }

                let x = tet.pivot[0];
                let y = tet.pivot[1];
                tet.pivot[0] = y;
                tet.pivot[1] = -x;

                // TODO: Try to help the player instead of just disallowing rotation
                if collision_check(&tet, &occupied, 0, 0) {
                    for i in 0..=3 {
                        let x = tet.model[i][0];
                        let y = tet.model[i][1];

                        tet.model[i][0] = -y;
                        tet.model[i][1] = x;
                    }

                    let x = tet.pivot[0];
                    let y = tet.pivot[1];
                    tet.pivot[0] = -y;
                    tet.pivot[1] = x;
                }

                print_tet(&mut tet, false);
            }
        }
    }
}

fn setup() {
    stdout().execute(Hide).unwrap();
    move_cursor(0, 0);

    let term_size = terminal::size().unwrap();

    for i in 0..term_size.1 {
        for j in 0..term_size.0 {
            if j <= W && i <= H {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn move_tet(tet: &mut Tet, x: i16, y: i16) {
    print_tet(tet, true);
    tet.pos[0] += x;
    tet.pos[1] += y;
    print_tet(tet, false);
}

fn print_tet(tet: &mut Tet, remove: bool) {
    let pos = tet_pos(&tet);
    for i in 0..=3 {
        let y: i16 = pos[1] + tet.model[i][1];
        if y < 0 {
            continue;
        }

        move_cursor(
            (pos[0] + tet.model[i][0]) as u16,
            y as u16
        );

        if remove {
            print!(".");
        } else {
            print!("■");
        }

        move_cursor(0, 0);
    }
}

fn collision_check(tet: &Tet, occupied: &Vec<[i16; 2]>, x: i16, y: i16) -> bool {
    let mut pos = tet_pos(&tet);
    pos[0] += x;
    pos[1] += y;
    for i in 0..=3 {
        let point = tet.model[i];
        if point[1] + pos[1] == H as i16 {
            return true;
        }

        if point[0] + pos[0] == W as i16 + 1{
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

fn tet_pos(tet: &Tet) -> [i16; 2] {
    return [tet.pos[0] - tet.pivot[0], tet.pos[1] - tet.pivot[1]];
}

fn place_tet(tet: &Tet, occupied: &mut Vec<[i16; 2]>) {
    let pos = tet_pos(&tet);
    for i in 0..=3 {
        occupied.push([pos[0] + tet.model[i][0], pos[1] + tet.model[i][1]]);
    }
}

fn move_cursor(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).unwrap();
}

fn debug_print(y: u16, print: String) {
    move_cursor(30, y);
    print!("{}", print);
}
