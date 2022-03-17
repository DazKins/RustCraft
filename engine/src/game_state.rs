use cgmath::Vector3;

use crate::{input::InputState, RenderContext};

pub trait GameState {
    fn tick(&mut self, input_state: &InputState);
    fn render(&mut self, render_context: &mut RenderContext);
    fn get_camera_position(&self) -> Vector3<f32>;
    fn get_camera_rotation(&self) -> Vector3<f32>;
}
