use std::{time::{SystemTime, Duration}, io::stdout};
use crossterm::{event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind}, cursor::Hide, ExecutableCommand, terminal};
use tet::Tet;

mod generic;
mod tet;

fn main() {
    setup();

    let mut occupied: Vec<[i16; 2]> = Vec::new();
    let mut tet = Tet::new_random();
    let mut time = SystemTime::now();

    loop {
        // Auto fall
        if time.elapsed().unwrap().as_secs() >= 1 {
            if tet.collision_check(&occupied, 0, 1) {
                tet.place(&mut occupied);
                tet = Tet::new_random();
            }

            tet.translate(0, 1);
            time = SystemTime::now();
        }

        // Controls
        if poll(Duration::from_secs(0)).unwrap() {
            let event = crossterm::event::read().unwrap();

            // Move left
            if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !tet.collision_check(&occupied, -1, 0) {
                    tet.translate(-1, 0);
                }
            // Move right
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !tet.collision_check(&occupied, 1, 0) {
                    tet.translate(1, 0);
                }
            // Move down
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Press)) {
                if !tet.collision_check(&occupied, 0, 1) {
                    tet.translate(0, 1);
                }
                if tet.collision_check(&occupied, 0, 1) {
                    tet.place(&mut occupied);
                    tet = Tet::new_random();
                }
                time = SystemTime::now();
            // Rotate left
            } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::NONE, KeyEventKind::Press)) {
                // TODO: Fix weird flipping on i, z, s, o
                tet.print(true);
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
                if tet.collision_check(&occupied, 0, 0) {
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

                tet.print(false);
            }
        }
    }
}

fn setup() {
    stdout().execute(Hide).unwrap();
    generic::move_cursor(0, 0);

    let term_size = terminal::size().unwrap();

    for i in 0..term_size.1 {
        for j in 0..term_size.0 {
            if j <= generic::W && i <= generic::H {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
