use std::{ffi::c_void, mem::zeroed, ptr::null};

use glad_gl::gl::{self, GLchar, GLsizei, GLuint};
use glfw::{self, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

// Vertex shader source is in source code for now
const VERTEX_SHADER_SOURCE: &str = concat!(
    "#version 330 core\n",
    "layout (location = 0) in vec3 aPos;\n",
    "void main()\n",
    "{\n",
    "   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n",
    "}\n\0",
);
const FRAGMENT_SHADER_SOURCE: &str = concat!(
    "#version 330 core\n",
    "out vec4 FragColor;\n",
    "void main()\n",
    "{\n",
    "   FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n",
    "}\n\0"
);

fn main() {
    // Initialize GLFW and window
    let (mut glfw_data, mut window, events_receiver) = {
        let mut glfw_data = glfw::init_no_callbacks().unwrap();

        // Ask for version 3.3 and the core profile
        glfw_data.window_hint(WindowHint::ContextVersionMajor(3));
        glfw_data.window_hint(WindowHint::ContextVersionMinor(3));
        glfw_data.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        // Create a window object
        let (mut window, events_receiver) = glfw_data
            .create_window(800, 600, "LearnOpenGL", WindowMode::Windowed)
            .unwrap();

        window.make_current();

        // Load opengl object pointers
        gl::load(|e| glfw_data.get_proc_address_raw(e).unwrap() as *const std::os::raw::c_void);

        // Set up viewport
        unsafe {
            gl::Viewport(0, 0, 800, 600);
        }

        (glfw_data, window, events_receiver)
    };

    // TODO: register resize callback

    // Create the shader program
    let shader_program = {
        // Build and compile our vertex shader
        let vertex_shader = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                VERTEX_SHADER_SOURCE.as_ptr() as *const *const GLchar,
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
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                FRAGMENT_SHADER_SOURCE.as_ptr() as *const *const i8,
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
            gl::GetProgramiv(shader_program, gl::COMPILE_STATUS, &mut success as *mut i32);
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

        shader_program
    };

    // Vertex input
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, //top left
    ];
    let vbo = {
        let mut vbo: GLuint = 0;
        unsafe {
            // Generate a vertex buffer object
            gl::GenBuffers(1, &mut vbo as *mut GLuint);

            // Set vbo type to ARRAY_BUFFER for vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Copy the data into the buffer
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of::<f32>() * vertices.len()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW, // Data is set once and used many times
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * size_of::<f32>()) as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);
        }
        vbo
    };

    while !window.should_close() {
        window.swap_buffers();
        glfw_data.poll_events();

        for (_, event) in glfw::flush_messages(&events_receiver) {
            match event {
                WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
