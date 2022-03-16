use engine::TextureCoordinate;

use crate::game::world::block::block::Block;

pub struct BlockTexture {
    pub top_face_texture_location: TextureCoordinate,
    pub bottom_face_texture_location: TextureCoordinate,
    pub north_face_texture_location: TextureCoordinate,
    pub south_face_texture_location: TextureCoordinate,
    pub west_face_texture_location: TextureCoordinate,
    pub east_face_texture_location: TextureCoordinate,
}

pub fn get_block_texture(block: Block) -> BlockTexture {
    match block {
        Block::Air => BlockTexture {
            top_face_texture_location: TextureCoordinate::new(0, 0),
            bottom_face_texture_location: TextureCoordinate::new(0, 0),
            north_face_texture_location: TextureCoordinate::new(0, 0),
            south_face_texture_location: TextureCoordinate::new(0, 0),
            west_face_texture_location: TextureCoordinate::new(0, 0),
            east_face_texture_location: TextureCoordinate::new(0, 0),
        },
        Block::Grass => BlockTexture {
            top_face_texture_location: TextureCoordinate::new(0, 0),
            bottom_face_texture_location: TextureCoordinate::new(64, 0),
            north_face_texture_location: TextureCoordinate::new(32, 0),
            south_face_texture_location: TextureCoordinate::new(32, 0),
            west_face_texture_location: TextureCoordinate::new(32, 0),
            east_face_texture_location: TextureCoordinate::new(32, 0),
        },
    }
}
