use cgmath::{Vector2};
use engine::{input::InputState, noise::Noise};

use crate::game::world::block::block::{Block};


pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_HEIGHT: u32 = 128;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkCoordinate {
    pub x: i32,
    pub z: i32
}

pub struct ChunkBlockCoordinate {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

impl ChunkBlockCoordinate {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        ChunkBlockCoordinate {
            x: x.rem_euclid(CHUNK_SIZE),
            y: y.rem_euclid(CHUNK_HEIGHT),
            z: z.rem_euclid(CHUNK_SIZE)
        }
    }
}

pub struct Chunk {
    pub position: ChunkCoordinate,
    blocks: Vec<Vec<Vec<Block>>>
}

impl Chunk {
    pub fn new(chunk_coordinate: ChunkCoordinate, noise: &mut Noise) -> Self {
        let mut blocks = vec![vec![vec![Block::Air; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let worldx = (chunk_coordinate.x * CHUNK_SIZE as i32) + x as i32;
                let worldz = (chunk_coordinate.z * CHUNK_SIZE as i32) + z as i32;

                let sample = noise.sample(Vector2::new((worldx as f32) / (CHUNK_SIZE as f32), (worldz as f32) / (CHUNK_SIZE as f32)));

                let height = (sample * 100.0 + 60.0).clamp(0.0, CHUNK_HEIGHT as f32) as u32;

                for y in 0..height {
                    blocks[x as usize][y as usize][z as usize] = Block::Grass;
                }
            }
        }

        Chunk {
            position: chunk_coordinate,
            blocks: blocks
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
    }

    pub fn get_block(&self, chunk_block_coordinate: ChunkBlockCoordinate) -> Block {
        self.blocks
            [chunk_block_coordinate.x as usize]
            [chunk_block_coordinate.y as usize]
            [chunk_block_coordinate.z as usize]
    }
}
