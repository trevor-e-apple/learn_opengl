use std::{
    ffi::c_void,
    mem::zeroed,
    ptr::null,
};

use glad_gl::gl::{self, GLchar, GLsizei, GLuint};
use glfw::{self, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

// Vertex shader source is in source code for now
const VERTEX_SHADER_SOURCE: &str = concat!(
    "#version 330 core\n",
    "layout (location = 0) in vec3 aPos;\n",
    "layout (location = 1) in vec3 aColor;\n",
    "out vec3 ourColor;\n",
    "void main()\n",
    "{\n",
    "   gl_Position = vec4(aPos, 1.0);\n",
    "   ourColor = aColor;\n",
    "}\n\0",
);
const FRAGMENT_SHADER_ONE_SOURCE: &str = concat!(
    "#version 330 core\n",
    "in vec3 ourColor;\n",
    "out vec4 FragColor;\n",
    "void main()\n",
    "{\n",
    "   FragColor = vec4(ourColor, 1.0);\n",
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
            let vertex_shader_source = {
                let boxed_source_ptr = Box::new(VERTEX_SHADER_SOURCE.as_ptr());
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
        let fragment_shader_one = unsafe {
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_shader_source = {
                let boxed_source_ptr = Box::new(FRAGMENT_SHADER_ONE_SOURCE.as_ptr());
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
            gl::AttachShader(shader_program, fragment_shader_one);
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
            gl::DeleteShader(fragment_shader_one);
        }

        shader_program
    };

    // Vertex input
    let triangle_one: [f32; 18] = [
        // 3 position, 3 color
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0,// Vertex 1
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // Vertex 2
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // Vertex 3
    ];
    let vaos = {
        let mut vbos: [GLuint; 2] = [0, 0];
        let mut vaos: [GLuint; 2] = [0, 0];
        unsafe {
            // Generate a vertex array object
            gl::GenVertexArrays(2, vaos.as_mut_ptr() as *mut GLuint);

            // Generate a vertex buffer object
            gl::GenBuffers(2, vbos.as_mut_ptr() as *mut GLuint);

            // Bind vertex array object first
            gl::BindVertexArray(vaos[0]);

            // Set vbo type to ARRAY_BUFFER for vertices
            gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);

            // Copy the data into the buffer
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of::<f32>() * triangle_one.len()) as isize,
                triangle_one.as_ptr() as *const c_void,
                gl::STATIC_DRAW, // Data is set once and used many times
            );

            // position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as i32,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            // color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as i32,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        vaos
    };

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events_receiver) {
            match event {
                WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        unsafe {
            // clear the color buffer
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw_data.poll_events();
    }
}
