use std::time::{SystemTime, Duration};
use crossterm::event::{poll, Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
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

        // Game over
        generic::move_cursor(28, 10);
        println!("Game over");
        generic::move_cursor(28, 11);
        println!("Press ENTER to try again,");
        generic::move_cursor(28, 12);
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
