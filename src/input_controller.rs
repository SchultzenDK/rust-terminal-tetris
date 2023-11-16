use crossterm::event::KeyCode;
use crate::{input::Input, input_mem::InputMem};

pub struct InputController {
    input: Input,
    input_memory: [InputMem; 3], // TODO: Refactor to vec
}

impl InputController {
    pub fn new() -> InputController {
        InputController {
            input: Input::new(),
            input_memory: [
                InputMem::new(KeyCode::Left),
                InputMem::new(KeyCode::Right),
                InputMem::new(KeyCode::Down)
            ]
        }
    }

    /// Call at start of every cycle
    pub fn update(&mut self) {
        self.input.capture_input();
    }

    /// Call at end of every cyle
    pub fn end_update(&mut self) {
        for mem in &mut self.input_memory {
            mem.set_released(&self.input);
        }
    }

    pub fn key_hold(&mut self, code: KeyCode) -> bool {
        let index: usize = match code {
            KeyCode::Left => 0,
            KeyCode::Right => 1,
            KeyCode::Down => 2,
            _ => 255
        };

        if index == 255 {
            return false;
        }

        let down = self.input.key_down(code) && self.input_memory[index].allowed();
        if down {
            self.input_memory[index].set_time();
        }

        down
    }

    pub fn key_pressed(&self, code: KeyCode) -> bool {
        self.input.key_pressed(code)
    }

    pub fn wait_for_enter() {
        let mut input_controller = InputController::new();

        loop {
            input_controller.update();

            if input_controller.key_pressed(KeyCode::Enter) {
                return;
            }
        }
    }
}