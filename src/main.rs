mod shader;

use std::{ffi::c_void, path::Path};

use glad_gl::gl::{self, GLuint};
use glfw::{self, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

use crate::shader::ShaderProgram;

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
    let shader_program =
        ShaderProgram::new(Path::new("./src/shader.vs"), Path::new("./src/shader.fs"));

    // Vertex input
    let triangle_one: [f32; 18] = [
        // 3 position, 3 color
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // Vertex 1
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

            shader_program.use_program();
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw_data.poll_events();
    }
}
