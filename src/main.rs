use std::time::SystemTime;
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
    // TODO: Figure out if I wanna keep GameController, or if I should make main have all that shit
    let mut game_controller = GameController::new();

    loop {
        let mut input_controller = InputController::new();

        game_controller.reset();
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
                game_controller.time = SystemTime::now();
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
