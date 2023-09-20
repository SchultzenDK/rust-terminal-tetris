use crossterm::event::KeyCode;
use generic::move_cursor;
use crate::{tet::Tet, game_controller::GameController, input_controller::InputController};

mod generic;
mod tet;
mod point;
mod input;
mod board;
mod input_mem;
mod game_controller;
mod input_controller;

fn main() {
    init();
    let mut input_controller = InputController::new();

    loop {
        let menu_options: [MenuOption; 3] = [
            MenuOption::New(String::from("New Game")),
            MenuOption::Score(String::from("View Highscores")),
            MenuOption::Quit(String::from("Quit")),
        ];

        let selected_option = main_menu_loop(&menu_options, &mut input_controller);

        match selected_option {
            MenuOption::New(_) => game_loop(&mut input_controller),
            MenuOption::Quit(_) => break,
            _ => {
                generic::clear_terminal();
                move_cursor(3, 10);
                println!("Not available yet");
            },
        }
    }

    generic::clear_terminal();
    generic::move_cursor(0, 0);
    generic::hide_cursor(false);
}

fn init() {
    generic::hide_cursor(true);
    generic::clear_terminal();
}

fn main_menu_loop<'a>(menu_options: &'a [MenuOption], input_controller: &mut InputController) -> &'a MenuOption {
    generic::move_cursor(3, 1);
    println!("Rust Terminal Tetris");

    let menu_options_offset = 3;
    update_cursor(0, 0, menu_options_offset);

    for (i, option) in menu_options.iter().enumerate() {
        generic::move_cursor(5, menu_options_offset + i as u16);
        println!("{}", match option {
            MenuOption::New(label) => label,
            MenuOption::Score(label) => label,
            MenuOption::Quit(label) => label,
        });
    }

    let mut cursor_index: u16 = 0;

    loop {
        input_controller.update();

        let prev_cursor_index = cursor_index;
        if input_controller.key_pressed(KeyCode::Down) {
            if cursor_index + 1 == menu_options.len() as u16 {
                cursor_index = 0
            } else {
                cursor_index += 1;
            }
        }
        if input_controller.key_pressed(KeyCode::Up) {
            if cursor_index == 0 {
                cursor_index = menu_options.len() as u16 - 1;
            } else {
                cursor_index -= 1;
            }
        }

        if prev_cursor_index != cursor_index {
            update_cursor(prev_cursor_index, cursor_index, menu_options_offset);
        }
        if input_controller.key_pressed(KeyCode::Enter) {
            return &menu_options[cursor_index as usize];
        }
    }
}

fn update_cursor(prev_y: u16, y: u16, offset: u16) {
    generic::move_cursor(3, prev_y + offset);
    println!(" ");
    generic::move_cursor(3, y + offset);
    println!(">");
}

fn game_loop(input_controller: &mut InputController) {
    let mut game_controller = GameController::new();
    let mut tet = Tet::new_random();

    loop {
        input_controller.update();

        if input_controller.key_pressed(KeyCode::Esc) {
            GameController::game_over(input_controller);
            return;
        }

        // Auto fall
        if game_controller.should_autofall() {
            if !tet.move_down(&mut game_controller) {
                break;
            }
        }

        if input_controller.key_hold(KeyCode::Left) {
            tet.translate(-1, 0, &game_controller);
        }
        if input_controller.key_hold(KeyCode::Right) {
            tet.translate(1, 0, &game_controller);
        }
        if input_controller.key_hold(KeyCode::Down) {
            if !tet.move_down(&mut game_controller) {
                break;
            }
            game_controller.reset_time();
        }

        if input_controller.key_pressed(KeyCode::Up) {
            tet.rotate(&game_controller);
        }

        input_controller.end_update();
    }

    GameController::game_over(input_controller);
}

enum MenuOption {
    New(String),
    Score(String),
    Quit(String),
}
