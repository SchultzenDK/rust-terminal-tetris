use std::{time::{SystemTime, Duration}, io::stdin};
use crossterm::event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use tet::Tet;
use point::Point;
use input::Input;

mod generic;
mod tet;
mod point;
mod input;

fn main() {
    // Required for running EXE directly
    println!("Press enter to start");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut input = Input::new();

    loop {
        setup();

        let mut occupied: Vec<Point> = Vec::new();
        let mut tet = Tet::new_random();
        let mut time = SystemTime::now();
        let mut input_memory = [
            InputMem::new(KeyCode::Left),
            InputMem::new(KeyCode::Right),
            InputMem::new(KeyCode::Down)
        ];
        let mut score: u32 = 0;
        print_score(score);

        loop {
            // Auto fall
            if time.elapsed().unwrap().as_secs() >= 1 {
                if !move_tet_down(&mut tet, &mut occupied, &mut score) {
                    break;
                }
                time = SystemTime::now();
            }

            // Controls
            input.capture_input();

            if input.key_down(KeyCode::Left) && input_memory[0].allowed() {
                tet.translate(-1, 0, &occupied);
                input_memory[0].set_time();
            }
            if input.key_down(KeyCode::Right) && input_memory[1].allowed() {
                tet.translate(1, 0, &occupied);
                input_memory[1].set_time();
            }
            if input.key_down(KeyCode::Down) && input_memory[2].allowed() {
                if !move_tet_down(&mut tet, &mut occupied, &mut score) {
                    break;
                }
                time = SystemTime::now();
                input_memory[2].set_time();
            }

            if input.key_pressed(KeyCode::Up) {
                tet.rotate(&occupied);
            }

            for mem in &mut input_memory {
                mem.set_released(&input);
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

fn update_score(cleared_rows: u8, score: &mut u32) {
    *score += 3_u32.pow(cleared_rows as u32 + 1);
    print_score(*score);
}

fn print_score(score: u32) {
    generic::move_cursor(15, 2);
    println!("Score: {:?}", score);
}

/// Move tet down
///
/// Returns false if unable to place, otherwise true
fn move_tet_down(tet: &mut Tet, occupied: &mut Vec<Point>, score: &mut u32) -> bool {
    if !tet.translate(0, 1, &occupied) {
        if !tet.place(&mut *occupied) {
            return false;
        }

        let cleared_rows = clear_full_rows(&mut *occupied);
        update_score(cleared_rows, &mut *score);
        *tet = Tet::new_random();
    }

    return true;
}

/// Clear rows that span entire width of board
///
/// Returns cleared row count
fn clear_full_rows(occupied: &mut Vec<Point>) -> u8 {
    let rows = get_row_count(occupied);

    // Which rows should move down, and how far
    let mut move_down_arr: [u8; generic::H as usize] = [0; generic::H as usize];
    let mut cleared_rows: u8 = 0;
    for i in (0..move_down_arr.len()).rev() {
        if rows[i] == generic::W {
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

    cleared_rows
}

/// Get all rows with count of occupied spaces
fn get_row_count(occupied: &Vec<Point>) -> [u16; generic::H as usize] {
    let mut rows: [u16; generic::H as usize] = [0; generic::H as usize];

    for occ in occupied {
        // NOTE: Panics if Y is negative (which it is when Tet spawns)
        rows[occ.y as usize] += 1;
    }

    // TODO: Remove `return` where not needed
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

struct InputMem {
    code: KeyCode,
    time: SystemTime,
    released: bool,
}

impl InputMem {
    fn new(code: KeyCode) -> InputMem {
        InputMem {
            code,
            time: SystemTime::now(),
            released: true,
        }
    }

    fn allowed(&self) -> bool {
        self.released || self.time.elapsed().unwrap().as_millis() > 100
    }

    fn set_time(&mut self) {
        self.time = SystemTime::now();
    }

    fn set_released(&mut self, input: &Input) {
        self.released = !input.key_down(self.code);
    }
}
