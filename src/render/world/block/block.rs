use cgmath::Vector3;
use engine::model::ModelBuilder;

use crate::game::world::block::block::Block;

use super::block_textures::get_block_texture;

const BLOCK_TEXTURE_SIZE: u32 = 32;

pub struct BlockRenderer;

impl BlockRenderer {
    fn generate_south_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).south_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_north_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).north_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    fn generate_east_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).east_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_west_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).west_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    fn generate_top_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).top_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_bottom_face(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let t00 = get_block_texture(block).bottom_face_texture_location;
        let t10 = t00.add_x(BLOCK_TEXTURE_SIZE);
        let t01 = t00.add_y(BLOCK_TEXTURE_SIZE);
        let t11 = t10.add_y(BLOCK_TEXTURE_SIZE);

        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    pub fn generate(
        &self,
        model_builder: &mut ModelBuilder,
        block: Block,
        x: u32,
        y: u32,
        z: u32,
        north_block: Block,
        south_block: Block,
        east_block: Block,
        west_block: Block,
        top_block: Block,
        bottom_block: Block,
    ) {
        if block == Block::Air {
            return;
        }

        if !north_block.get_block_properties().is_solid {
            self.generate_north_face(model_builder, block, x, y, z);
        }
        if !south_block.get_block_properties().is_solid {
            self.generate_south_face(model_builder, block, x, y, z);
        }
        if !east_block.get_block_properties().is_solid {
            self.generate_east_face(model_builder, block, x, y, z);
        }
        if !west_block.get_block_properties().is_solid {
            self.generate_west_face(model_builder, block, x, y, z);
        }
        if !top_block.get_block_properties().is_solid {
            self.generate_top_face(model_builder, block, x, y, z);
        }
        if !bottom_block.get_block_properties().is_solid {
            self.generate_bottom_face(model_builder, block, x, y, z);
        }
    }
}
