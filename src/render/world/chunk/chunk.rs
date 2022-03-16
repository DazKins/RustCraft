use std::collections::HashMap;

use cgmath::{Matrix4, Vector3};
use engine::{RenderContext, model::{Model, ModelBuilder}, Texture};

use crate::{game::world::{chunk::{Chunk, CHUNK_SIZE, ChunkCoordinate, CHUNK_HEIGHT, ChunkBlockCoordinate}, block::block::Block}, render::world::block::block::BlockRenderer};

pub struct ChunkRenderer {
    block_renderer: BlockRenderer,
    model_cache: HashMap<ChunkCoordinate, Model>,
    texture: Texture
}

impl ChunkRenderer {
    pub fn new() -> Self {
        ChunkRenderer {
            block_renderer: BlockRenderer,
            model_cache: HashMap::new(),
            texture: Texture::new("block.png")
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext, chunk: &Chunk,
        north_chunk: Option<&Chunk>, south_chunk: Option<&Chunk>,
        east_chunk: Option<&Chunk>, west_chunk: Option<&Chunk>
    ) {
        if self.model_cache.get(&chunk.position).is_none() {
            self.model_cache.insert(chunk.position, self.generate_model(chunk, north_chunk, south_chunk, east_chunk, west_chunk));
        }

        let world_x = (CHUNK_SIZE as i32 * chunk.position.x) as f32;
        let world_z = (CHUNK_SIZE as i32 * chunk.position.z) as f32;

        render_context.get_matrix_stack().push();
        render_context.get_matrix_stack().transform(&Matrix4::from_translation(Vector3::new(world_x, 0.0, world_z)));

        render_context.render(self.model_cache.get(&chunk.position).unwrap());

        render_context.get_matrix_stack().pop();
    }

    pub fn generate_model(&self, chunk: &Chunk,
        north_chunk: Option<&Chunk>, south_chunk: Option<&Chunk>,
        east_chunk: Option<&Chunk>, west_chunk: Option<&Chunk>
    ) -> Model {
        let mut model_builder = ModelBuilder::new(self.texture);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let north_block = if z < CHUNK_SIZE - 1 {
                        chunk.get_block(ChunkBlockCoordinate{ x, y, z: z + 1 })
                    } else {
                        match north_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(x, y, 0)),
                            None => Block::Air
                        }
                    };

                    let south_block = if z > 0 {
                        chunk.get_block(ChunkBlockCoordinate{ x, y, z: z - 1 })
                    } else {
                        match south_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(x, y, CHUNK_SIZE - 1)),
                            None => Block::Air
                        }
                    };

                    let east_block = if x < CHUNK_SIZE - 1 {
                        chunk.get_block(ChunkBlockCoordinate{ x: x + 1, y, z })
                    } else {
                        match east_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(0, y, z)),
                            None => Block::Air
                        }
                    };

                    let west_block = if x > 0 {
                        chunk.get_block(ChunkBlockCoordinate{ x: x - 1, y, z })
                    } else {
                        match west_chunk {
                            Some(chunk) => chunk.get_block(ChunkBlockCoordinate::new(CHUNK_SIZE - 1, y, z)),
                            None => Block::Air
                        }
                    };

                    let top_block = if y < CHUNK_HEIGHT - 1 {
                        chunk.get_block(ChunkBlockCoordinate{ x, y: y + 1, z })
                    } else {
                        Block::Air
                    };

                    let bottom_block = if y > 0 {
                        chunk.get_block(ChunkBlockCoordinate{ x, y: y - 1, z })
                    } else {
                        Block::Air
                    };

                    let block = chunk.get_block(ChunkBlockCoordinate{ x, y, z });

                    self.block_renderer.generate(
                        &mut model_builder,
                        block,
                        x, y, z,
                        north_block, south_block,
                        east_block, west_block,
                        top_block, bottom_block
                    );
                }
            }
        }

        model_builder.build()
    }
}