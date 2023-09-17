use std::time::SystemTime;
use crossterm::event::KeyCode;
use crate::input::Input;

pub struct InputMem {
    code: KeyCode,
    time: SystemTime,
    released: bool,
}

impl InputMem {
    pub fn new(code: KeyCode) -> InputMem {
        InputMem {
            code,
            time: SystemTime::now(),
            released: true,
        }
    }

    pub fn allowed(&self) -> bool {
        self.released || self.time.elapsed().unwrap().as_millis() > 100
    }

    pub fn set_time(&mut self) {
        self.time = SystemTime::now();
    }

    pub fn set_released(&mut self, input: &Input) {
        self.released = !input.key_down(self.code);
    }
}