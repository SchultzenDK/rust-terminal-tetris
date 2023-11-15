use crossterm::event::KeyCode;

use crate::{input_controller::InputController, generic};

#[derive(Clone)]
pub enum MenuOption {
    New(String),
    Score(String),
    Quit(String),
}

pub fn main_menu_loop(input_controller: &mut InputController) -> MenuOption {
    let menu_options: [MenuOption; 3] = [
        MenuOption::New(String::from("New Game")),
        MenuOption::Score(String::from("View Highscores")),
        MenuOption::Quit(String::from("Quit")),
    ];

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
            return menu_options[cursor_index as usize].clone();
        }
    }
}

fn update_cursor(prev_y: u16, y: u16, offset: u16) {
    generic::move_cursor(3, prev_y + offset);
    println!(" ");
    generic::move_cursor(3, y + offset);
    println!(">");
}
