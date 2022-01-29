use cgmath::Vector3;
use engine::{model::ModelBuilder, TextureCoordinate};

use crate::game::world::chunk::{ChunkCoordinate, CHUNK_SIZE, ChunkBlockCoordinate, CHUNK_HEIGHT};

const BLOCK_TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
pub struct Block {
    is_solid: bool,

    top_tex_loc: TextureCoordinate,
    bottom_tex_loc: TextureCoordinate,
    front_tex_loc: TextureCoordinate,
    back_tex_loc: TextureCoordinate,
    left_tex_loc: TextureCoordinate,
    right_tex_loc: TextureCoordinate,
}

impl Block {
    fn generate_back_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.back_tex_loc;
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
            .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_front_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.front_tex_loc;
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
            .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    fn generate_right_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.right_tex_loc;
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
            .set_xyz(Vector3::new( 1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(1.0 + x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_left_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.left_tex_loc;
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
            .set_xyz(Vector3::new( x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    fn generate_top_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.top_tex_loc;
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
            .set_xyz(Vector3::new( 1.0 + x as f32, 1.0 + y as f32, z as f32))
            .set_texcoord(t10)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, 1.0 + y as f32, 1.0 + z as f32))
            .set_texcoord(t01)
            .push_vertex();
    }

    fn generate_bottom_face(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        let t00 = self.bottom_tex_loc;
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
            .set_xyz(Vector3::new( 1.0 + x as f32, y as f32, z as f32))
            .set_texcoord(t00)
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(x as f32, y as f32, 1.0 + z as f32))
            .set_texcoord(t11)
            .push_vertex();
    }

    pub fn generate(&self, model_builder: &mut ModelBuilder, x: u32, y: u32, z: u32) {
        if self.is_solid {
            self.generate_back_face(model_builder, x, y, z);
            self.generate_front_face(model_builder, x, y, z);
            self.generate_right_face(model_builder, x, y, z);
            self.generate_left_face(model_builder, x, y, z);
            self.generate_top_face(model_builder, x, y, z);
            self.generate_bottom_face(model_builder, x, y, z);
        }
    }
}

pub const BLOCK_AIR: Block = Block {
    is_solid: false,

    top_tex_loc: TextureCoordinate::new(0,0),
    bottom_tex_loc: TextureCoordinate::new(0,0),
    front_tex_loc: TextureCoordinate::new(0,0),
    back_tex_loc: TextureCoordinate::new(0,0),
    left_tex_loc: TextureCoordinate::new(0,0),
    right_tex_loc: TextureCoordinate::new(0,0),
};

pub const BLOCK_GRASS: Block = Block {
    is_solid: true,

    top_tex_loc: TextureCoordinate::new(0,0),
    bottom_tex_loc: TextureCoordinate::new(64,0),
    front_tex_loc: TextureCoordinate::new(32,0),
    back_tex_loc: TextureCoordinate::new(32,0),
    left_tex_loc: TextureCoordinate::new(32,0),
    right_tex_loc: TextureCoordinate::new(32,0),
};

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
        ChunkCoordinate::new(
            self.x / (CHUNK_SIZE as i32),
            self.x / (CHUNK_SIZE as i32),
        )
    }

    pub fn to_chunk_block_coordinate(&self) -> ChunkBlockCoordinate {
        ChunkBlockCoordinate::new(
            self.x.rem_euclid(CHUNK_SIZE as i32) as u32,
            self.y.rem_euclid(CHUNK_HEIGHT as i32) as u32,
            self.z.rem_euclid(CHUNK_SIZE as i32) as u32
        )
    }
}
