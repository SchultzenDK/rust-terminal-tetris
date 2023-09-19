use std::io::stdin;
use crossterm::event::KeyCode;
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

    loop {
        let mut game_controller = GameController::new();
        let mut input_controller = InputController::new();
        let mut tet = Tet::new_random();

        loop {
            input_controller.update();

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

        if GameController::game_over(&mut input_controller) {
            break;
        }
    }
}

fn init() {
    generic::hide_cursor(true);

    // Required for running EXE directly
    println!("Press enter to start");
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
