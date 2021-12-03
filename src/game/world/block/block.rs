use cgmath::{Vector3, Vector2};
use engine::model::ModelBuilder;

#[derive(Clone, Copy)]
pub struct Block {
    is_solid: bool
}

impl Block {
    pub fn generate(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        if self.is_solid {
            // Back Face
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

            // Front Face
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

            // Right Face
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( 1.0 + x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

            // Left Face
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

            // Top Face
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( 1.0 + x as f32, 1.0 + y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

            // Bottom Face
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 1.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new( 1.0 + x as f32, y as f32, z as f32))
                .set_uv(Vector2::new(0.0, 0.0))
                .push_vertex();
            model_builder
                .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
                .set_uv(Vector2::new(1.0, 1.0))
                .push_vertex();

        }
    }
}

pub const BLOCK_AIR: Block = Block {
    is_solid: false
};
pub const BLOCK_STONE: Block = Block {
    is_solid: true
};
