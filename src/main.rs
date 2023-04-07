use std::{time::{SystemTime, Duration}};
use crossterm::event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use crossterm_cursor::TerminalCursor;

struct Tet {
    pos: [i16; 2],
    pivot: [i16; 2],
    model: [[i16; 2]; 4],
    rot: u8,
}

impl Tet {
    fn new() -> Tet {
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
}

fn main() {
    const H:i16 = 20;
    const W:i16 = 10;
    let cursor = TerminalCursor::new();
    setup(H as u16, W as u16, &cursor);

    let mut occupied: Vec<[i16; 2]> = Vec::new();
    let mut tet = Tet::new();
    let mut time = SystemTime::now();

    loop {
        let dur = time.elapsed().unwrap();
        if dur.as_secs() >= 1 {
            move_tet(&mut tet, &cursor, 0, 1);
            time = SystemTime::now();
        }

        if poll(Duration::from_secs(0)).unwrap() {
            let event = crossterm::event::read().unwrap();

            if tet.pos[0] > 0 && event == Event::Key(KeyEvent::new_with_kind(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press)) {
                move_tet(&mut tet, &cursor, -1, 0);
            } else if tet.pos[0] < W && event == Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::NONE, KeyEventKind::Press)) {
                move_tet(&mut tet, &cursor, 1, 0);
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Press)) {
                move_tet(&mut tet, &cursor, 0, 1);
                time = SystemTime::now();
            }
        }

        if below_collision(&tet, &H, &occupied) {
            let pos = tet_pos(&tet);
            for i in 0..=3 {
                occupied.push([pos[0] + tet.model[i][0], pos[1] + tet.model[i][1]]);
            }
            tet = Tet::new();
        }


        cursor.goto(0, 0).unwrap();
    }
}

fn setup(h: u16, w: u16, cursor: &TerminalCursor) {
    cursor.hide().unwrap();
    cursor.goto(0, 0).unwrap();

    let term_size = crossterm::terminal::size().unwrap();

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

fn move_tet(tet: &mut Tet, cursor: &TerminalCursor, x: i16, y: i16) {
    print_tet(tet, cursor, true);
    tet.pos[0] += x;
    tet.pos[1] += y;
    print_tet(tet, cursor, false);
}

fn print_tet(tet: &mut Tet, cursor: &TerminalCursor, remove: bool) {
    let pos = tet_pos(&tet);
    for i in 0..=3 {
        let y: i16 = pos[1] + tet.model[i][1];
        if y < 0 {
            continue;
        }
        cursor.goto((pos[0] + tet.model[i][0]) as u16, y as u16).unwrap();
        if remove {
            print!(".");
        } else {
            print!("■");
            // DEBUG for printing pivot point
            if y == tet.pos[1] {
                cursor.goto((pos[0] + tet.model[i][0]) as u16, y as u16).unwrap();
                print!("X");
            }
        }
    }
}

fn below_collision(tet: &Tet, h: &i16, occupied: &Vec<[i16; 2]>) -> bool {
    let pos = tet_pos(&tet);
    for i in 0..=3 {
        let point = tet.model[i];
        if point[1] + pos[1] == h - 1 {
            return true;
        }

        for occ in occupied {
            if point[0] + pos[0] == occ[0] && point[1] + pos[1] == occ[1] - 1 {
                return true;
            }
        }
    }

    return false;
}

fn tet_pos(tet: &Tet) -> [i16; 2] {
    return [tet.pos[0] - tet.pivot[0], tet.pos[1] - tet.pivot[1]];
}
