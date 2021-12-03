use cgmath::Vector2;
use engine::{input::InputState, model::{ModelBuilder, Model}, RenderContext, Texture};

use rand::Rng;

use crate::game::world::{Block, block::{ BLOCK_AIR, BLOCK_STONE }};

const CHUNK_WIDTH: usize = 16;
const CHUNK_DEPTH: usize = 16;
const CHUNK_HEIGHT: usize = 128;

pub struct Chunk {
    position: Vector2<f32>,
    blocks: [[[Block; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
    model: Option<Model>,
    texture: Texture
}

impl Chunk {
    pub fn new() -> Self {
        let mut blocks = [[[BLOCK_AIR; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

        let mut rng = rand::thread_rng();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    if rng.gen_bool(0.01) {
                        blocks[x][y][z] = BLOCK_STONE;
                    }
                }
            }
        }

        Chunk {
            position: Vector2::new(0.0, 0.0),
            blocks,
            model: None,
            texture: Texture::new("container.jpg")
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
    }

    pub fn render(&mut self, render_context: &RenderContext) {
        self.texture.bind();

        if self.model.is_none() {
            self.model = Option::Some(self.generate())
        }

        match &self.model {
            Some(model) => render_context.render(model),
            None        => (),
        }
    }

    pub fn generate(&self) -> Model {
        let mut model_builder = ModelBuilder::new();

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    self.blocks[x][y][z].generate(&mut model_builder, x as u32, y as u32, z as u32);
                }
            }
        }

        model_builder.build()
    }
}
