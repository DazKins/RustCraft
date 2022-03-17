use cgmath::Vector2;

use crate::input::Key;
use std::collections::HashMap;

pub struct InputState {
    keys_pressed: HashMap<Key, bool>,
    mouse_position: Vector2<f32>,
    mouse_speed: Vector2<f32>,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            keys_pressed: HashMap::new(),
            mouse_position: Vector2::new(0.0, 0.0),
            mouse_speed: Vector2::new(0.0, 0.0),
        }
    }

    pub fn set_pressed(&mut self, key: Key) {
        self.keys_pressed.insert(key, true);
    }

    pub fn set_released(&mut self, key: Key) {
        self.keys_pressed.insert(key, false);
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        return self.keys_pressed.get(&key).cloned().unwrap_or(false);
    }

    pub fn set_mouse_position(&mut self, position: &Vector2<f32>) {
        self.mouse_position = position.clone();
    }

    pub fn set_mouse_speed(&mut self, speed: &Vector2<f32>) {
        self.mouse_speed = speed.clone();
    }

    pub fn get_mouse_position(&self) -> Vector2<f32> {
        return self.mouse_position;
    }

    pub fn get_mouse_speed(&self) -> Vector2<f32> {
        return self.mouse_speed;
    }
}
