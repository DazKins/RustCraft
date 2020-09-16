use cgmath::{Vector3, Matrix4, perspective, Deg, Rad};

use crate::input::{InputState, Key};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>
}

const MOVE_SPEED: f32 = 0.1;

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        let mouse_speed = input_state.get_mouse_speed();

        self.rotation.y = self.rotation.y + mouse_speed.x * 0.01;
        self.rotation.x = self.rotation.x + mouse_speed.y * 0.01;

        let (sin_y, cos_y) = self.rotation.y.sin_cos();

        if input_state.is_key_pressed(Key::W) {
            self.position.z = self.position.z + cos_y * MOVE_SPEED;
            self.position.x = self.position.x - sin_y * MOVE_SPEED;
        } 
        if input_state.is_key_pressed(Key::S) {
            self.position.z = self.position.z - cos_y * MOVE_SPEED;
            self.position.x = self.position.x + sin_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::A) {
            self.position.z = self.position.z + sin_y * MOVE_SPEED;
            self.position.x = self.position.x + cos_y * MOVE_SPEED;
        }
        if input_state.is_key_pressed(Key::D) {
            self.position.z = self.position.z - sin_y * MOVE_SPEED;
            self.position.x = self.position.x - cos_y * MOVE_SPEED;
        }
    }

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        let perspective: Matrix4<f32> = perspective( Deg(60.0f32), 1.3, 0.1, 1000.0).into();
        let position = Matrix4::from_translation(self.position);

        let rotation = Matrix4::from_angle_x(Rad(self.rotation.x)) *
            Matrix4::from_angle_y(Rad(self.rotation.y)) *
            Matrix4::from_angle_z(Rad(self.rotation.z));

        return perspective * rotation * position;
    }
}
