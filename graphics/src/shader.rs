use gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;

pub struct Shader {
    program_id: GLuint
}

impl Shader {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Shader {
        let program_id = unsafe {
            let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader_id);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vertex_shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("Vertex shader compilation failed. Reason: {}", str::from_utf8(&info_log).unwrap());
            }

            let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader_id, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader_id);

            gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(fragment_shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("Fragment shader compilation failed. Reason: {}", str::from_utf8(&info_log).unwrap());
            }

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("Shader program linking failed. Reason: {}", str::from_utf8(&info_log).unwrap());
            }

            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);

            program_id
        };

        Shader {
            program_id
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}
