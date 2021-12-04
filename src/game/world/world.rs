use engine::{RenderContext, input::InputState};

use super::chunk::{Chunk, ChunkCoordinate};

pub struct World {
    chunks: [Chunk; 4]
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: [
                Chunk::new(ChunkCoordinate::new(0, 0)),
                Chunk::new(ChunkCoordinate::new(1, 0)),
                Chunk::new(ChunkCoordinate::new(0, 1)),
                Chunk::new(ChunkCoordinate::new(1, 1))
            ]
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        for chunk in self.chunks.iter_mut() {
            chunk.tick(input_state);
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext) {
        for chunk in self.chunks.iter_mut() {
            chunk.render(render_context);
        }
    }
}
