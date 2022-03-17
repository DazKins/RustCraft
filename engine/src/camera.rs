use cgmath::{perspective, Deg, Matrix4, Rad, Vector3};

use crate::input::{InputState, Key};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

const MOVE_SPEED: f32 = 0.1;

impl Camera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            fov,
            aspect,
            near,
            far,
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

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        let perspective: Matrix4<f32> =
            perspective(Deg(self.fov), self.aspect, self.near, self.far).into();
        let position = Matrix4::from_translation(-self.position);

        let rotation = Matrix4::from_angle_x(Rad(self.rotation.x))
            * Matrix4::from_angle_y(Rad(self.rotation.y))
            * Matrix4::from_angle_z(Rad(self.rotation.z));

        return perspective * rotation * position;
    }
}
