use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub struct Input {
    // TODO: maybe make a key struct
    // TODO: maybe remove `released` if you're not going to use it
    down: Vec<KeyCode>,
    pressed: Vec<KeyCode>,
    released: Vec<KeyCode>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            down: Vec::new(),
            pressed: Vec::new(),
            released: Vec::new(),
        }
    }

    /// Captures input and stores it
    ///
    /// Must be run every cycle, or input won't be read as expected
    pub fn capture_input(&mut self) {
        self.released.clear();
        self.pressed.clear();
        if crossterm::event::poll(Duration::from_secs(0)).unwrap() {
            if let Event::Key(KeyEvent { code, modifiers: _, kind, .. }) = crossterm::event::read().unwrap() {
                if kind == KeyEventKind::Release {
                    self.released.push(code);
                    let down_index = self.get_down_index(code);
                    if down_index > -1 {
                        self.down.remove(down_index as usize);
                    }
                } else if kind == KeyEventKind::Press && self.get_down_index(code) == -1 {
                    self.pressed.push(code);
                    self.down.push(code);
                }
            }
        }
    }

    pub fn key_down(&self, code: KeyCode) -> bool {
        self.get_down_index(code) > -1
    }

    pub fn key_pressed(&self, code: KeyCode) -> bool {
        self.get_pressed_index(code) > -1
    }

    // pub fn key_released(&self, code: KeyCode) -> bool {
    //     self.get_released_index(code) > -1
    // }

    /// Get index of `code` in `key_vec`
    ///
    /// Returns -1 on no index
    fn get_key_index(key_vec: &Vec<KeyCode>, code: KeyCode) -> isize {
        for i in 0..key_vec.len() {
            if key_vec[i] == code {
                return i as isize;
            }
        }

        -1
    }

    fn get_down_index(&self, code: KeyCode) -> isize {
        Input::get_key_index(&self.down, code)
    }

    fn get_pressed_index(&self, code: KeyCode) -> isize {
        Input::get_key_index(&self.pressed, code)
    }

    // fn get_released_index(&self, code: KeyCode) -> isize {
    //     Input::get_key_index(&self.released, code)
    // }
}
