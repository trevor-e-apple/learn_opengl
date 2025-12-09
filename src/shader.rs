use std::{fs::read_to_string, mem::zeroed, path::Path, ptr::null};

use glad_gl::gl::{self, GLchar, GLfloat, GLsizei};

use crate::{matrix::Matrix4, vector::Vector3};

pub struct ShaderProgram {
    handle: u32,
}

impl ShaderProgram {
    pub fn new(vertex_path: &Path, fragment_path: &Path) -> Self {
        // Build and compile our vertex shader
        let vertex_shader = unsafe {
            let mut vertex_shader_source = read_to_string(vertex_path).unwrap();
            vertex_shader_source.push('\0');

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_shader_source = {
                let boxed_source_ptr = Box::new(vertex_shader_source.as_ptr());
                Box::into_raw(boxed_source_ptr)
            };
            gl::ShaderSource(
                vertex_shader,
                1,
                vertex_shader_source as *const *const GLchar,
                null(),
            );
            gl::CompileShader(vertex_shader);

            // Check for compilation errors
            let mut success: i32 = 0;
            let mut info_log: [u8; 512] = zeroed();
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success as *mut i32);
            if success == 0 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    0 as *mut GLsizei,
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!("Vertex shader compilation error");
                eprintln!("Error info: {}", str::from_utf8(&info_log).unwrap());
                panic!("panic");
            }

            vertex_shader
        };

        // Build and compile our fragment shader
        let fragment_shader = unsafe {
            let mut fragment_shader_source = read_to_string(fragment_path).unwrap();
            fragment_shader_source.push('\0');

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_shader_source = {
                let boxed_source_ptr = Box::new(fragment_shader_source.as_ptr());
                Box::into_raw(boxed_source_ptr)
            };
            gl::ShaderSource(
                fragment_shader,
                1,
                fragment_shader_source as *const *const GLchar,
                null(),
            );
            gl::CompileShader(fragment_shader);

            // Check for compilation errors
            let mut success: i32 = 0;
            let mut info_log: [u8; 512] = zeroed();
            gl::GetShaderiv(
                fragment_shader,
                gl::COMPILE_STATUS,
                &mut success as *mut i32,
            );
            if success == 0 {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    0 as *mut GLsizei,
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!("Fragment shader compilation error");
                eprintln!("Error info: {}", str::from_utf8(&info_log).unwrap());
                panic!("panic");
            }

            fragment_shader
        };

        // Create the shader program
        let shader_program = unsafe {
            let shader_program = gl::CreateProgram();

            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success: i32 = 0;
            let mut info_log: [u8; 512] = zeroed();
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success as *mut i32);
            if success == 0 {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    0 as *mut GLsizei,
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                eprintln!("Shader program linking error");
                eprintln!("Error info: {}", str::from_utf8(&info_log).unwrap());
                panic!("panic");
            }

            shader_program
        };

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Self {
            handle: shader_program,
        }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.handle) }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.handle, name.as_ptr() as *const GLchar),
                value,
            );
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.handle, name.as_ptr() as *const GLchar),
                value,
            );
        }
    }

    pub fn set_vec3(&self, name: &str, value: &Vector3) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.handle, name.as_ptr() as *const GLchar),
                value.x,
                value.y,
                value.z,
            );
        }
    }

    pub fn set_mat4(&self, name: &str, value: &Matrix4) {
        unsafe {
            let location = gl::GetUniformLocation(self.handle, name.as_ptr() as *const GLchar);
            let error = gl::GetError();
            assert_eq!(error, 0);
            gl::UniformMatrix4fv(location, 1, gl::TRUE, value.data.as_ptr() as *const GLfloat);
            let error = gl::GetError();
            assert_eq!(error, 0);
        }
    }
}
