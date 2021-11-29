use cgmath::{Vector3, Vector2};
use engine::model::ModelBuilder;

#[derive(Clone, Copy)]
pub struct Block {
    isSolid: bool
}

impl Block {
    pub fn generate(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        if self.isSolid {
            model_builder
                .set_xyz(Vector3::new(0.5 + x as f32, 0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(0.5 + x as f32, -0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(-0.5 + x as f32, 0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();

            model_builder
                .set_xyz(Vector3::new(-0.5 + x as f32, -0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(-0.5 + x as f32, 0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(0.5 + x as f32, -0.5 + y as f32, -1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

        }
    }
}

pub const BLOCK_AIR: Block = Block {
    isSolid: true
};
pub const BLOCK_STONE: Block = Block {
    isSolid: true
};
