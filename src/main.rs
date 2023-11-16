use crate::{game_controller::GameController, input_controller::InputController, menu_controller::MenuOption};

mod generic;
mod tet;
mod point;
mod input;
mod board;
mod input_mem;
mod game_controller;
mod input_controller;
mod menu_controller;
mod highscore_controller;

fn main() {
    init();
    let mut input_controller = InputController::new();

    loop {
        let selected_option = menu_controller::main_menu_loop(&mut input_controller);

        match selected_option {
            MenuOption::New(_) => new_game(&mut input_controller),
            MenuOption::Score(_) => menu_controller::score_menu_loop(&mut input_controller),
            MenuOption::Quit(_) => break
        }
    }

    cleanup_exit();
}

fn init() {
    generic::hide_cursor(true);
    generic::clear_terminal();
}

fn cleanup_exit() {
    generic::clear_terminal();
    generic::move_cursor(0, 0);
    generic::hide_cursor(false);
}

fn new_game(input_controller: &mut InputController) {
    let mut game_controller = GameController::new();
    game_controller.game_loop(input_controller);
}
