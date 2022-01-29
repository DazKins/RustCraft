use std::{collections::HashMap};

use engine::{RenderContext, input::InputState, noise::Noise};

use super::{chunk::{Chunk, ChunkCoordinate}, block::{BlockCoordinate, BLOCK_AIR}, Block};

const WORLD_SIZE: i32 = 8;

pub struct World {
    chunks: HashMap<ChunkCoordinate, Chunk>
}

impl World {
    pub fn new() -> Self {
        let mut noise = Noise::new(8, 1.7, 0.125);

        let mut chunks = HashMap::new();

        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                let chunk_coordinate = ChunkCoordinate::new(x, z);
                chunks.insert(chunk_coordinate, Chunk::new(chunk_coordinate, &mut noise));
            }
        }

        World {
            chunks
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        for (_, chunk) in self.chunks.iter_mut() {
            chunk.tick(input_state);
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext) {
        for (_, chunk) in self.chunks.iter_mut() {
            chunk.render(render_context);
        }
    }

    pub fn get_block(&self, block_coordinate: BlockCoordinate) -> Block {
        let chunk_coordinate = block_coordinate.to_chunk_coordinate();

        match self.chunks.get(&chunk_coordinate) {
            Some(chunk) => {
                let chunk_block_coordinate = block_coordinate.to_chunk_block_coordinate();
                chunk.get_block(chunk_block_coordinate)
            },
            None => BLOCK_AIR
        }
    }
}
