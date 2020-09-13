use crate::{RenderContext, input::InputState};

pub trait GameState {
    fn tick(&mut self, input_state: &InputState);
    fn render(&mut self, render_context: &mut RenderContext);
}
