use gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use crate::Texture;

const POSITION_ATTRIBUTE_LOCATION: u32 = 0;
const TEXTURE_ATTRIBUTE_LOCATION: u32 = 1;

const VERTEX_POSITION_SIZE: u32 = 3;
const VERTEX_TEXTURE_SIZE: u32 = 2;

const VERTEX_DATA_SIZE: u32 = VERTEX_POSITION_SIZE + VERTEX_TEXTURE_SIZE;

pub struct Model {
    vao_id: GLuint,
    index_count: i32,
    texture: Texture,
}

impl Model {
    pub fn new(vertices: &[f32], indices: &[i32], texture: Texture) -> Model {
        let vao_id = unsafe {
            let (mut vao_id, mut vbo_id, mut ebo_id) = (0, 0, 0);

            gl::GenVertexArrays(1, &mut vao_id);
            gl::GenBuffers(1, &mut vbo_id);
            gl::GenBuffers(1, &mut ebo_id);

            gl::BindVertexArray(vao_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            let mut offset: usize = 0;

            gl::VertexAttribPointer(
                POSITION_ATTRIBUTE_LOCATION,
                VERTEX_POSITION_SIZE as i32,
                gl::FLOAT,
                gl::FALSE,
                VERTEX_DATA_SIZE as i32 * mem::size_of::<GLfloat>() as GLsizei,
                (offset as usize * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(POSITION_ATTRIBUTE_LOCATION);
            offset += VERTEX_POSITION_SIZE as usize;

            gl::VertexAttribPointer(
                TEXTURE_ATTRIBUTE_LOCATION,
                VERTEX_TEXTURE_SIZE as i32,
                gl::FLOAT,
                gl::FALSE,
                VERTEX_DATA_SIZE as i32 * mem::size_of::<GLfloat>() as GLsizei,
                (offset as usize * mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(TEXTURE_ATTRIBUTE_LOCATION);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            vao_id
        };

        return Model {
            vao_id,
            index_count: indices.len() as i32,
            texture,
        };
    }

    pub fn render(&self) {
        self.texture.bind();
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
