use std::{cell::RefCell, ops::Deref};

use engine::RenderContext;

use crate::game::world::{chunk::ChunkCoordinate, world::World};

use super::chunk::chunk::ChunkRenderer;

pub struct WorldRenderer {
    chunk_renderer: ChunkRenderer
}

impl WorldRenderer {
    pub fn new() -> Self {
        WorldRenderer {
            chunk_renderer: ChunkRenderer::new()
        }
    }

    pub fn render(&mut self, render_context: &mut RenderContext, world: &World) {
        for (chunk_coordinate, chunk) in world.chunks.iter() {
            let north_chunk_coord = ChunkCoordinate {
                x: chunk_coordinate.x,
                z: chunk_coordinate.z + 1
            };
            let south_chunk_coord = ChunkCoordinate {
                x: chunk_coordinate.x,
                z: chunk_coordinate.z - 1
            };
            let east_chunk_coord = ChunkCoordinate {
                x: chunk_coordinate.x + 1,
                z: chunk_coordinate.z
            };
            let west_chunk_coord = ChunkCoordinate {
                x: chunk_coordinate.x - 1,
                z: chunk_coordinate.z
            };

            let north_chunk = world.chunks.get(&north_chunk_coord).map(RefCell::borrow);
            let south_chunk = world.chunks.get(&south_chunk_coord).map(RefCell::borrow);
            let east_chunk = world.chunks.get(&east_chunk_coord).map(RefCell::borrow);
            let west_chunk = world.chunks.get(&west_chunk_coord).map(RefCell::borrow);

            self.chunk_renderer.render(render_context, chunk.borrow().deref(),
                north_chunk.as_deref(), south_chunk.as_deref(),
                east_chunk.as_deref(), west_chunk.as_deref()
            )
        }
    }
}