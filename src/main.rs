use std::{time::{SystemTime, Duration}, io::stdin};
use crossterm::{event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind}};
use tet::Tet;
use point::Point;

mod generic;
mod tet;
mod point;

fn main() {
    // Required for running EXE directly
    println!("Press enter to start");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    loop {
        setup();

        let mut occupied: Vec<Point> = Vec::new();
        let mut tet = Tet::new_random();
        let mut time = SystemTime::now();

        loop {
            // Auto fall
            if time.elapsed().unwrap().as_secs() >= 1 {
                if !tet.translate(0, 1, &occupied) {
                    if !tet.place(&mut occupied) {
                        break;
                    }

                    clear_full_rows(&mut occupied);
                    tet = Tet::new_random();
                }

                time = SystemTime::now();
            }

            // Controls
            if poll(Duration::from_secs(0)).unwrap() {
                let event = crossterm::event::read().unwrap();

                // Move left
                if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Left, KeyModifiers::NONE, KeyEventKind::Press)) {
                    tet.translate(-1, 0, &occupied);
                // Move right
                } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Right, KeyModifiers::NONE, KeyEventKind::Press)) {
                    tet.translate(1, 0, &occupied);
                // Move down
                } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Press)) {
                    if !tet.translate(0, 1, &occupied) {
                        if !tet.place(&mut occupied) {
                            break;
                        }

                        clear_full_rows(&mut occupied);
                        tet = Tet::new_random();
                    }

                    time = SystemTime::now();
                // Rotate left
                } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::NONE, KeyEventKind::Press)) {
                    tet.rotate(&occupied);
                }
            }
        }

        // Game over
        generic::move_cursor(15, 10);
        println!("Game over");
        generic::move_cursor(15, 11);
        println!("Press ENTER to try again,");
        generic::move_cursor(15, 12);
        println!("or ESC to quit");

        loop {
            if poll(Duration::from_secs(0)).unwrap() {
                let event = crossterm::event::read().unwrap();

                if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Enter, KeyModifiers::NONE, KeyEventKind::Press)) {
                    break;
                } else if event == Event::Key(KeyEvent::new_with_kind(KeyCode::Esc, KeyModifiers::NONE, KeyEventKind::Press)) {
                    generic::clear_terminal();
                    generic::move_cursor(0, 0);
                    return;
                }
            }
        }
    }
}

fn setup() {
    generic::hide_cursor();
    generic::clear_terminal();
    generic::clear_board();
}

/// Clear rows that span entire width of board
fn clear_full_rows(occupied: &mut Vec<Point>) {
    let rows = get_row_count(occupied);

    // Which rows should move down, and how far
    let mut move_down_arr: [u16; generic::H as usize] = [0; generic::H as usize];
    let mut move_down: u16 = 0;
    for i in (0..move_down_arr.len()).rev() {
        if rows[i] == generic::W {
            move_down += 1;
        } else {
            move_down_arr[i] = move_down;
        }
    }

    // Nothing was moved down, so no rows were cleared
    if move_down == 0 {
        return;
    }

    // Move rows down and save rows to remove
    let mut indexes_to_remove: Vec<usize> = Vec::new();
    for i in 0..occupied.len() {
        // NOTE: Panics if Y is negative (which it is when Tet spawns)
        if rows[occupied[i].y as usize] == generic::W {
            indexes_to_remove.push(i);
        } else {
            occupied[i].y += move_down_arr[occupied[i].y as usize] as i16;
        }
    }

    // Remove rows
    for i in (indexes_to_remove).iter().rev() {
        occupied.remove(*i);
    }

    // Print updates
    print_occupied(occupied);
}

/// Get all rows with count of occupied spaces
fn get_row_count(occupied: &Vec<Point>) -> [u16; generic::H as usize] {
    let mut rows: [u16; generic::H as usize] = [0; generic::H as usize];

    for occ in occupied {
        // NOTE: Panics if Y is negative (which it is when Tet spawns)
        rows[occ.y as usize] += 1;
    }

    return rows;
}

/// Clear board and print occupied points
fn print_occupied(occupied: &Vec<Point>) {
    generic::clear_board();

    for occ in occupied {
        generic::move_cursor(occ.x as u16, occ.y as u16);
        print!("■");
    }

    generic::move_cursor(0, 0);
}
