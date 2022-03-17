use cgmath::Vector3;
use engine::input::{InputState, Key};

pub struct Player {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
}

const MOVE_SPEED: f32 = 0.1;

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        let mouse_speed = input_state.get_mouse_speed();

        self.rotation.y += mouse_speed.x * 0.002;
        self.rotation.x += mouse_speed.y * 0.002;

        let (sin_y, cos_y) = self.rotation.y.sin_cos();

        if input_state.is_key_pressed(Key::W) {
            self.position.z -= cos_y * MOVE_SPEED;
            self.position.x += sin_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::S) {
            self.position.z += cos_y * MOVE_SPEED;
            self.position.x -= sin_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::A) {
            self.position.z -= sin_y * MOVE_SPEED;
            self.position.x -= cos_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::D) {
            self.position.z += sin_y * MOVE_SPEED;
            self.position.x += cos_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::Space) {
            self.position.y += MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::LeftControl) {
            self.position.y -= MOVE_SPEED;
        }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        self.rotation
    }
}
