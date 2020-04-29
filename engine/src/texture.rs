extern crate image;

use std::path::Path;
use std::os::raw::c_void;

use image::GenericImage;

pub struct Texture {
    texture_id: u32
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

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img.width() as i32,
                img.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void);

            gl::GenerateMipmap(gl::TEXTURE_2D);

            texture_id
        };

        Texture {
            texture_id
        }
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
