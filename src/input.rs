use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub struct Input {
    // TODO: maybe make a key struct
    down: Vec<KeyCode>,
    pressed: Vec<KeyCode>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            down: Vec::new(),
            pressed: Vec::new(),
        }
    }

    /// Captures input and stores it
    ///
    /// Must be run every cycle, or input won't be read as expected
    pub fn capture_input(&mut self) {
        self.pressed.clear();
        if !crossterm::event::poll(Duration::from_secs(0)).unwrap() {
            return
        }

        if let Event::Key(KeyEvent { code, modifiers: _, kind, .. }) = crossterm::event::read().unwrap() {
            self.handle_key_event(code, kind);
        }
    }

    pub fn key_down(&self, code: KeyCode) -> bool {
        self.get_down_index(code).is_ok()
    }

    pub fn key_pressed(&self, code: KeyCode) -> bool {
        self.get_pressed_index(code).is_ok()
    }

    fn handle_key_event(&mut self, code: KeyCode, kind: KeyEventKind) {
        if kind == KeyEventKind::Release {
            self.handle_key_event_release(code);
        } else if kind == KeyEventKind::Press {
            self.handle_key_event_press(code);
        }
    }

    fn handle_key_event_release(&mut self, code: KeyCode) {
        let down_index_result = self.get_down_index(code);
        if let Ok(index) = down_index_result {
            self.down.remove(index);
        }
    }

    fn handle_key_event_press(&mut self, code: KeyCode) {
        if self.get_down_index(code).is_ok() {
            return
        }

        self.pressed.push(code);
        self.down.push(code);
    }

    /// Get index of `code` in `key_vec`
    fn get_key_index(key_vec: &Vec<KeyCode>, code: KeyCode) -> Result<usize, usize> {
        for i in 0..key_vec.len() {
            if key_vec[i] == code {
                return Ok(i);
            }
        }

        Err(0)
    }

    fn get_down_index(&self, code: KeyCode) -> Result<usize, usize> {
        Input::get_key_index(&self.down, code)
    }

    fn get_pressed_index(&self, code: KeyCode) -> Result<usize, usize> {
        Input::get_key_index(&self.pressed, code)
    }
}
