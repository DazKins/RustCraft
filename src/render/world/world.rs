use std::{cell::RefCell, ops::Deref};

use engine::RenderContext;

use crate::game::world::{chunk::{ChunkCoordinate, CHUNK_SIZE}, world::World};

use super::chunk::chunk::ChunkRenderer;

const CHUNK_RENDER_RADIUS: i32 = 16;

pub struct WorldRenderer {
    chunk_renderer: ChunkRenderer,
}

impl WorldRenderer {
    pub fn new() -> Self {
        WorldRenderer {
            chunk_renderer: ChunkRenderer::new(),
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext, world: &World) {
        let x0 = (world.get_player().get_position().x / (CHUNK_SIZE as f32)) as i32;
        let z0 = (world.get_player().get_position().z / (CHUNK_SIZE as f32)) as i32;

        for x in (x0 - CHUNK_RENDER_RADIUS)..(x0 + CHUNK_RENDER_RADIUS) {
            for z in (z0 - CHUNK_RENDER_RADIUS)..(z0 + CHUNK_RENDER_RADIUS) {
                let chunk_coordinate = ChunkCoordinate { x, z };

                let chunk_option = world.chunks.get(&chunk_coordinate);

                if chunk_option.is_none() {
                    continue;
                }

                let chunk = chunk_option.unwrap();

                let north_chunk_coordinate = ChunkCoordinate {
                    x,
                    z: z + 1,
                };
                let south_chunk_coordinate = ChunkCoordinate {
                    x,
                    z: z - 1,
                };
                let east_chunk_coordinate = ChunkCoordinate {
                    x: x + 1,
                    z,
                };
                let west_chunk_coordinate = ChunkCoordinate {
                    x: x - 1,
                    z,
                };

                let north_chunk = world.chunks.get(&north_chunk_coordinate).map(RefCell::borrow);
                let south_chunk = world.chunks.get(&south_chunk_coordinate).map(RefCell::borrow);
                let east_chunk = world.chunks.get(&east_chunk_coordinate).map(RefCell::borrow);
                let west_chunk = world.chunks.get(&west_chunk_coordinate).map(RefCell::borrow);

                self.chunk_renderer.render(
                    render_context,
                    chunk.borrow().deref(),
                    north_chunk.as_deref(),
                    south_chunk.as_deref(),
                    east_chunk.as_deref(),
                    west_chunk.as_deref(),
                )
            }
        }
    }
}
