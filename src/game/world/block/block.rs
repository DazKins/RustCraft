use crate::game::world::chunk::{ChunkCoordinate, CHUNK_SIZE, ChunkBlockCoordinate, CHUNK_HEIGHT};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Grass
}

pub struct BlockProperties {
    pub is_solid: bool
}

impl Block {
    pub fn get_block_properties(self) -> BlockProperties {
        match self {
            Block::Air => BlockProperties {
                is_solid: false
            },
            Block::Grass => BlockProperties {
                is_solid: true
            }
        }
    }
}

pub struct BlockCoordinate {
    x: i32,
    y: i32,
    z: i32
}

impl BlockCoordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        BlockCoordinate {
            x, y, z
        }
    }

    pub fn to_chunk_coordinate(&self) -> ChunkCoordinate {
        ChunkCoordinate {
            x: self.x / (CHUNK_SIZE as i32),
            z: self.z / (CHUNK_SIZE as i32),
        }
    }

    pub fn to_chunk_block_coordinate(&self) -> ChunkBlockCoordinate {
        ChunkBlockCoordinate::new(
            self.x.rem_euclid(CHUNK_SIZE as i32) as u32,
            self.y.rem_euclid(CHUNK_HEIGHT as i32) as u32,
            self.z.rem_euclid(CHUNK_SIZE as i32) as u32
        )
    }
}
