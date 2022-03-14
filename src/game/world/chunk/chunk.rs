use cgmath::{Vector2, Matrix4, Vector3};
use engine::{input::InputState, model::{ModelBuilder, Model}, RenderContext, Texture, noise::Noise};

use crate::game::world::{Block, block::{BLOCK_GRASS, BLOCK_AIR}, World};

pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_HEIGHT: u32 = 128;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ChunkCoordinate {
    x: i32,
    z: i32
}

impl ChunkCoordinate {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkCoordinate {
            x, z
        }
    }

    pub fn add_x(&self, x: i32) -> ChunkCoordinate {
        ChunkCoordinate {
            x: self.x + x,
            z: self.z
        }
    }

    pub fn add_z(&self, z: i32) -> ChunkCoordinate {
        ChunkCoordinate {
            x: self.x,
            z: self.z + z
        }
    }
}

pub struct ChunkBlockCoordinate {
    x: u32,
    y: u32,
    z: u32
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
    position: ChunkCoordinate,
    blocks: Vec<Vec<Vec<Block>>>,
    model: Option<Model>,
    texture: Texture
}

impl Chunk {
    pub fn new(chunk_coordinate: ChunkCoordinate, noise: &mut Noise) -> Self {
        let mut blocks = vec![vec![vec![BLOCK_AIR; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let worldx = (chunk_coordinate.x * CHUNK_SIZE as i32) + x as i32;
                let worldz = (chunk_coordinate.z * CHUNK_SIZE as i32) + z as i32;

                let sample = noise.sample(Vector2::new((worldx as f32) / (CHUNK_SIZE as f32), (worldz as f32) / (CHUNK_SIZE as f32)));

                let height = (sample * 50.0 + 60.0).clamp(0.0, CHUNK_HEIGHT as f32) as u32;

                for y in 0..height {
                    blocks[x as usize][y as usize][z as usize] = BLOCK_GRASS;
                }
            }
        }

        Chunk {
            position: chunk_coordinate,
            blocks: blocks,
            model: None,
            texture: Texture::new("block.png")
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
    }

    pub fn render(&mut self, render_context: &mut RenderContext, north_chunk: Option<&Chunk>, south_chunk: Option<&Chunk>, east_chunk: Option<&Chunk>, west_chunk: Option<&Chunk>) {
        if self.model.is_none() {
            self.model = Some(self.generate(north_chunk, south_chunk, east_chunk, west_chunk))
        }

        let world_x = (CHUNK_SIZE as i32 * self.position.x) as f32;
        let world_z = (CHUNK_SIZE as i32 * self.position.z) as f32;

        render_context.get_matrix_stack().push();
        render_context.get_matrix_stack().transform(&Matrix4::from_translation(Vector3::new(world_x, 0.0, world_z)));
        match &self.model {
            Some(model) => render_context.render(model),
            None        => (),
        }
        render_context.get_matrix_stack().pop();
    }

    pub fn generate(&self, north_chunk: Option<&Chunk>, south_chunk: Option<&Chunk>, east_chunk: Option<&Chunk>, west_chunk: Option<&Chunk>) -> Model {
        let mut model_builder = ModelBuilder::new(self.texture);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let north_block = if z < CHUNK_SIZE - 1 {
                        self.blocks[x as usize][y as usize][z as usize + 1]
                    } else {
                        match north_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(x, y, 0)),
                            None => BLOCK_AIR
                    }
                    };

                    let south_block = if z > 0 {
                        self.blocks[x as usize][y as usize][z as usize - 1]
                    } else {
                        match south_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(x, y, CHUNK_SIZE - 1)),
                            None => BLOCK_AIR
                    }
                    };

                    let east_block = if x < CHUNK_SIZE - 1 {
                        self.blocks[x as usize + 1][y as usize][z as usize]
                    } else {
                        match east_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(0, y, z)),
                            None => BLOCK_AIR
                    }
                    };

                    let west_block = if x > 0 {
                        self.blocks[x as usize - 1][y as usize][z as usize]
                    } else {
                        match west_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(CHUNK_SIZE - 1, y, z)),
                            None => BLOCK_AIR
                    }
                    };

                    let top_block = if y < CHUNK_HEIGHT - 1 {
                        self.blocks[x as usize][y as usize + 1][z as usize]
                    } else {
                        BLOCK_AIR
                    };

                    let bottom_block = if y > 0 {
                        self.blocks[x as usize][y as usize - 1][z as usize]
                    } else {
                        BLOCK_AIR
                    };

                    self.blocks[x as usize][y as usize][z as usize].generate(
                        &mut model_builder,
                        x as u32, y as u32, z as u32,
                        north_block, south_block,
                        east_block, west_block,
                        top_block, bottom_block
                    );
                }
            }
        }

        model_builder.build()
    }

    pub fn get_block(&self, chunk_block_coordinate: ChunkBlockCoordinate) -> Block {
        self.blocks
            [chunk_block_coordinate.x as usize]
            [chunk_block_coordinate.y as usize]
            [chunk_block_coordinate.z as usize]
    }
}
