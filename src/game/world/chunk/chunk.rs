use cgmath::{Vector2, Matrix4, Vector3};
use engine::{input::InputState, model::{ModelBuilder, Model}, RenderContext, Texture, noise::Noise};

use crate::game::world::{Block, block::{BLOCK_GRASS, BLOCK_AIR}};

const CHUNK_SIZE: u32 = 16;
const CHUNK_HEIGHT: u32 = 128;

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
}

pub struct Chunk {
    position: ChunkCoordinate,
    blocks: Box<[[[Block; (CHUNK_SIZE as usize)]; (CHUNK_HEIGHT as usize)]; (CHUNK_SIZE as usize)]>,
    model: Option<Model>,
    texture: Texture
}

impl Chunk {
    pub fn new(chunk_coordinate: ChunkCoordinate, noise: &mut Noise) -> Self {
        let mut blocks = Box::new([[[BLOCK_AIR; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize]);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let worldx = (chunk_coordinate.x * CHUNK_SIZE as i32) + x as i32;
                let worldz = (chunk_coordinate.z * CHUNK_SIZE as i32) + z as i32;

                let sample = noise.sample(Vector2::new((worldx as f32) / (CHUNK_SIZE as f32), (worldz as f32) / (CHUNK_SIZE as f32)));

                let height = sample * 20.0 + 30.0;

                for y in 0..CHUNK_HEIGHT {
                    if y as f32 > height {
                        continue;
                    }

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

    pub fn render(&mut self, render_context: &mut RenderContext) {
        if self.model.is_none() {
            self.model = Option::Some(self.generate())
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

    pub fn generate(&self) -> Model {
        let mut model_builder = ModelBuilder::new(self.texture);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    self.blocks[x as usize][y as usize][z as usize].generate(&mut model_builder, x as u32, y as u32, z as u32);
                }
            }
        }

        model_builder.build()
    }
}
