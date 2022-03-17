use gl::types::*;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use cgmath::{Matrix, Matrix4};

pub struct Shader {
    program_id: GLuint,
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Shader {
        let program_id = unsafe {
            let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert =
                CString::new(load_string_from_file(vertex_shader_path).as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader_id);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "Vertex shader compilation failed. Reason: {}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag =
                CString::new(load_string_from_file(fragment_shader_path).as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader_id, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader_id);

            gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "Fragment shader compilation failed. Reason: {}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    program_id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "Shader program linking failed. Reason: {}",
                    str::from_utf8(&info_log).unwrap()
                );
            }

            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);

            program_id
        };

        Shader { program_id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform_mat4(&self, location: &str, matrix: &Matrix4<f32>) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.program_id, CString::new(location).unwrap().as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}

pub fn load_string_from_file(path: &str) -> String {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {}", path));
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Failed to read file");
    string
}
