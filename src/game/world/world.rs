use std::{cell::RefCell, collections::HashMap};

use engine::{input::InputState, noise::Noise};

use crate::game::entity::player::Player;

use super::{
    block::block::{Block, BlockCoordinate},
    chunk::{Chunk, ChunkCoordinate},
};

const WORLD_SIZE: i32 = 32;

pub struct World {
    pub chunks: HashMap<ChunkCoordinate, RefCell<Chunk>>,
    player: Player,
}

impl World {
    pub fn new() -> Self {
        let mut noise = Noise::new(16, 1.8, 1.0 / 32.0);

        let mut chunks = HashMap::new();

        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                let chunk_coordinate = ChunkCoordinate { x, z };
                let chunk = Chunk::new(chunk_coordinate, &mut noise);
                chunks.insert(chunk_coordinate, RefCell::new(chunk));
            }
        }

        World {
            chunks,
            player: Player::new(),
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        self.player.tick(input_state);
    }

    pub fn get_block(&self, block_coordinate: BlockCoordinate) -> Block {
        let chunk_coordinate = block_coordinate.to_chunk_coordinate();

        match self.chunks.get(&chunk_coordinate) {
            Some(chunk) => {
                let chunk_block_coordinate = block_coordinate.to_chunk_block_coordinate();
                chunk.borrow().get_block(chunk_block_coordinate)
            }
            None => Block::Air,
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
}
