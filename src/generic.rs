use std::io::stdout;
use crossterm::{cursor, terminal, Command, ExecutableCommand, style::{Color, SetForegroundColor}};

use crate::input_controller::InputController;

pub fn term_command(command: impl Command) {
    stdout().execute(command).unwrap();
}

pub fn move_cursor(x: u16, y: u16) {
    term_command(cursor::MoveTo(x, y));
}

#[allow(dead_code)]
pub fn debug_print(y: u16, print: &str) {
    move_cursor(40, y);
    print!("{}", print);
    move_cursor(0, 0);
}

pub fn error_print(msg: &str) {
    move_cursor(0, 0);
    set_color(Color::Red);
    println!("{}", msg);
    println!("Press ENTER to continue");
    set_color(Color::Reset);

    InputController::wait_for_enter();
}

pub fn clear_terminal() {
    term_command(terminal::Clear(terminal::ClearType::All));
}

pub fn hide_cursor(hide: bool) {
    if hide {
        term_command(cursor::Hide);
    } else {
        term_command(cursor::Show);
    }
}

pub fn set_color(color: Color) {
    term_command(SetForegroundColor(color));
}
