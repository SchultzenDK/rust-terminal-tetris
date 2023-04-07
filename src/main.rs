use std::{time::{SystemTime, Duration}, io::stdout};
use crossterm::{event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind}, cursor::{MoveTo, Hide}, ExecutableCommand, terminal};

struct Tet {
    pos: [i16; 2],
    pivot: [i16; 2],
    model: [[i16; 2]; 4],
    rot: u8,
}

impl Tet {
    fn new_line() -> Tet {
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
            rot: 0,
        }
    }

    fn new_l() -> Tet {
        Tet {
            pos: [
                5, 0
            ],
            pivot: [
                0, 2
            ],
            model: [
                [0, 0],
                [0, 1],
                [0, 2],
                [1, 2]
            ],
            rot: 0,
        }
    }
}

fn main() {
    const H:i16 = 20;
    const W:i16 = 10;
    setup(H as u16, W as u16);

    let mut occupied: Vec<[i16; 2]> = Vec::new();
    let mut tet = Tet::new_l();
    let mut time = SystemTime::now();

    loop {
        // Auto fall
        if time.elapsed().unwrap().as_secs() >= 1 {
            if collision_check(&tet, H, W, &occupied, 0, -1) {
                place_tet(&tet, &mut occupied);
                tet = Tet::new_l();
            }

            move_tet(&mut tet, 0, 1);
            time = SystemTime::now();
        }

        // Controls
        if poll(Duration::from_secs(0)).unwrap() {
            let event = crossterm::event::read().unwrap();

            if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, H, W, &occupied, 1, 0) {
                    move_tet(&mut tet, -1, 0);
                }
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, H, W, &occupied, -1, 0) {
                    move_tet(&mut tet, 1, 0);
                }
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !collision_check(&tet, H, W, &occupied, 0, -1) {
                    move_tet(&mut tet, 0, 1);
                }
                if collision_check(&tet, H, W, &occupied, 0, -1) {
                    place_tet(&tet, &mut occupied);
                    tet = Tet::new_l();
                }
                time = SystemTime::now();
            }
        }
    }
}

fn setup(h: u16, w: u16) {
    stdout().execute(Hide).unwrap();
    move_cursor(0, 0);

    let term_size = terminal::size().unwrap();

    for i in 0..term_size.1 {
        for j in 0..term_size.0 {
            if j <= w && i <= h {
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

fn collision_check(tet: &Tet, h: i16, w: i16, occupied: &Vec<[i16; 2]>, x: i16, y: i16) -> bool {
    let pos = tet_pos(&tet);
    for i in 0..=3 {
        let point = tet.model[i];
        if y != 0 && point[1] + pos[1] == h + y {
            return true;
        }

        if x < 0 && point[0] + pos[0] == w {
            return true;
        } else if x > 0 && point[0] + pos[0] == 0 {
            return true;
        }

        for occ in occupied {
            if point[0] + pos[0] == occ[0] + x && point[1] + pos[1] == occ[1] + y {
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
