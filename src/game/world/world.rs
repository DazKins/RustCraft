use std::{collections::HashMap, cell::RefCell};

use engine::{RenderContext, input::InputState, noise::Noise};

use super::{chunk::{Chunk, ChunkCoordinate}, block::{BlockCoordinate, BLOCK_AIR}, Block};

const WORLD_SIZE: i32 = 8;

pub struct World {
    chunks: HashMap<ChunkCoordinate, RefCell<Chunk>>
}

impl World {
    pub fn new() -> Self {
        let mut noise = Noise::new(8, 1.7, 0.125);

        let mut chunks = HashMap::new();

        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                let chunk_coordinate = ChunkCoordinate::new(x, z);
                let chunk = Chunk::new(chunk_coordinate, &mut noise);
                chunks.insert(chunk_coordinate, RefCell::new(chunk));
            }
        }

        World {
            chunks
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        for (_, chunk) in self.chunks.iter() {
            chunk.borrow_mut().tick(input_state);
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext) {
        for (chunk_coordinate, chunk) in self.chunks.iter() {
            let north_chunk = self.chunks.get(&chunk_coordinate.add_z(1)).map(RefCell::borrow);
            let south_chunk = self.chunks.get(&chunk_coordinate.add_z(-1)).map(RefCell::borrow);
            let east_chunk = self.chunks.get(&chunk_coordinate.add_x(1)).map(RefCell::borrow);
            let west_chunk = self.chunks.get(&chunk_coordinate.add_x(-1)).map(RefCell::borrow);

            chunk.borrow_mut().render(render_context, north_chunk.as_deref(), south_chunk.as_deref(), east_chunk.as_deref(), west_chunk.as_deref());
        }
    }

    pub fn get_block(&self, block_coordinate: BlockCoordinate) -> Block {
        let chunk_coordinate = block_coordinate.to_chunk_coordinate();

        match self.chunks.get(&chunk_coordinate) {
            Some(chunk) => {
                let chunk_block_coordinate = block_coordinate.to_chunk_block_coordinate();
                chunk.borrow().get_block(chunk_block_coordinate)
            },
            None => BLOCK_AIR
        }
    }
}
