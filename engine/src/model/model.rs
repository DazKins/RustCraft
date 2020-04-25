use gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct Model {
    vao_id: GLuint,
    index_count: i32
}

impl Model {
    pub fn new(vertices: &[f32], indices: &[i32]) -> Model {
        let vao_id = unsafe {
            let (mut vao_id, mut vbo_id, mut ebo_id) = (0, 0, 0);
    
            gl::GenVertexArrays(1, &mut vao_id);
            gl::GenBuffers(1, &mut vbo_id);
            gl::GenBuffers(1, &mut ebo_id);
    
            gl::BindVertexArray(vao_id);
    
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);
    
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &indices[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW);
    
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(0);
    
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    
            gl::BindVertexArray(0);
    
            vao_id
        };
    
        return Model {
            vao_id,
            index_count: indices.len() as i32
        };
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::DrawElements(gl::TRIANGLES, self.index_count as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }
}