use std::os::raw::c_void;
use std::path::Path;

use cgmath::Vector2;
use image::GenericImage;

#[derive(Clone, Copy)]
pub struct Texture {
    width: u32,
    height: u32,
    texture_id: u32,
}

#[derive(Clone, Copy)]
pub struct TextureCoordinate {
    x: u32,
    y: u32,
}

impl TextureCoordinate {
    pub const fn new(x: u32, y: u32) -> Self {
        TextureCoordinate { x, y }
    }

    pub fn add_x(&self, x: u32) -> TextureCoordinate {
        TextureCoordinate {
            x: self.x + x,
            y: self.y,
        }
    }

    pub fn add_y(&self, y: u32) -> TextureCoordinate {
        TextureCoordinate {
            x: self.x,
            y: self.y + y,
        }
    }
}

impl Texture {
    pub fn new(path: &str) -> Texture {
        let img = image::open(&Path::new(path)).unwrap();
        let data = img.raw_pixels();

        let texture_id = unsafe {
            let mut texture_id = 0;
            gl::GenTextures(1, &mut texture_id);

            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            texture_id
        };

        Texture {
            width: img.width(),
            height: img.height(),
            texture_id,
        }
    }

    pub fn get_uv(&self, tex_coord: TextureCoordinate) -> Vector2<f32> {
        Vector2::new(
            tex_coord.x as f32 / self.width as f32,
            tex_coord.y as f32 / self.height as f32,
        )
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
