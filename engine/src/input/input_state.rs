use crate::input::Key;
use std::collections::HashMap;

pub struct InputState{
    keys_pressed: HashMap<Key, bool>
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            keys_pressed: HashMap::new()
        }
    }

    pub fn set_pressed(&mut self, key: Key) {
        self.keys_pressed.insert(key, true);
    }

    pub fn set_released(&mut self, key: Key) {
        self.keys_pressed.insert(key, false);
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        return self.keys_pressed.get(&key).cloned().unwrap_or(false)
    }
}