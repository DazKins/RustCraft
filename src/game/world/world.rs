use engine::{RenderContext, input::InputState};

use super::chunk::Chunk;

pub struct World {
    chunk: Chunk
}

impl World {
    pub fn new() -> Self {
        World {
            chunk: Chunk::new()
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        self.chunk.tick(input_state);
    }

    pub fn render(&mut self, render_context: &mut RenderContext) {
        self.chunk.render(render_context);
    }
}
