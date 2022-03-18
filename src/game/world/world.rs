use std::{cell::RefCell, collections::HashMap};

use engine::{input::InputState, noise::Noise};

use crate::game::entity::player::Player;

use super::{
    block::block::{Block, BlockCoordinate},
    chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE},
};

const CHUNK_LOAD_RADIUS: i32 = 16;

pub struct World {
    pub chunks: HashMap<ChunkCoordinate, RefCell<Chunk>>,
    player: Player,
    noise: Noise,
}

impl World {
    pub fn new() -> Self {
        World {
            chunks: HashMap::new(),
            player: Player::new(),
            noise: Noise::new(16, 1.8, 1.0 / 32.0),
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        self.player.tick(input_state);

        let x0 = (self.player.get_position().x / (CHUNK_SIZE as f32)) as i32;
        let z0 = (self.player.get_position().z / (CHUNK_SIZE as f32)) as i32;

        for x in (x0 - CHUNK_LOAD_RADIUS)..(x0 + CHUNK_LOAD_RADIUS) {
            for z in (z0 - CHUNK_LOAD_RADIUS)..(z0 + CHUNK_LOAD_RADIUS) {
                let chunk_coordinate = ChunkCoordinate{ x, z };
                if !self.chunks.contains_key(&chunk_coordinate) {
                    self.chunks.insert(chunk_coordinate, RefCell::new(Chunk::new(chunk_coordinate, &mut self.noise)));
                }
            }
        }
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
