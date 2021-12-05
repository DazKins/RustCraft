use engine::{RenderContext, input::InputState, noise::Noise};

use super::chunk::{Chunk, ChunkCoordinate};

const WORLD_SIZE: i32 = 8;

pub struct World {
    chunks: Vec<Chunk>
}

impl World {
    pub fn new() -> Self {
        let mut noise = Noise::new();

        let mut chunks = Vec::new();

        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                chunks.push(Chunk::new(ChunkCoordinate::new(x, z), &mut noise))
            }
        }

        World {
            chunks
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
