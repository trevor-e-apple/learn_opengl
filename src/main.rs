mod camera;
mod math;
mod matrix;
mod shader;
mod vector;

use std::{ffi::c_void, path::Path, time::Instant};

use glad_gl::gl::{self, GLsizei, GLuint};
use glfw::{self, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};
use image::ImageReader;

use crate::{
    camera::Camera,
    math::angle_to_rad,
    matrix::{Matrix4, make_projection_matrix},
    shader::ShaderProgram,
    vector::Vector3,
};

fn scene_one() {
    // Initialize GLFW and window
    let width = 800;
    let height = 600;
    let (mut glfw_data, mut window, events_receiver) = {
        let mut glfw_data = glfw::init_no_callbacks().unwrap();

        // Ask for version 3.3 and the core profile
        glfw_data.window_hint(WindowHint::ContextVersionMajor(3));
        glfw_data.window_hint(WindowHint::ContextVersionMinor(3));
        glfw_data.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        // Create a window object
        let (mut window, events_receiver) = glfw_data
            .create_window(width, height, "LearnOpenGL", WindowMode::Windowed)
            .unwrap();

        window.make_current();

        // Load opengl object pointers
        gl::load(|e| glfw_data.get_proc_address_raw(e).unwrap() as *const std::os::raw::c_void);

        // Set up viewport
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::DEPTH_TEST);
        }

        (glfw_data, window, events_receiver)
    };

    // TODO: register resize callback
    let shader_program =
        ShaderProgram::new(Path::new("./src/shader.vs"), Path::new("./src/shader.fs"));

    // Vertex input
    let triangle_one: [f32; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5,
        -0.5, 1.0, 1.0, -0.5, 0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 0.0, -0.5, -0.5, 0.5,
        0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5,
        0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, 0.5, 0.5, 1.0, 0.0, -0.5, 0.5, -0.5,
        1.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, -0.5, 0.5, 0.0,
        0.0, -0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5,
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5,
        1.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, -0.5, 1.0, 1.0, 0.5, -0.5, 0.5, 1.0, 0.0,
        0.5, -0.5, 0.5, 1.0, 0.0, -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, 1.0, -0.5, 0.5,
        -0.5, 0.0, 1.0, 0.5, 0.5, -0.5, 1.0, 1.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0,
    ];
    let indices: [i32; 36] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    ];
    let vaos = {
        let mut vbos: [GLuint; 2] = [0, 0];
        let mut vaos: [GLuint; 2] = [0, 0];
        let mut ebos: [GLuint; 2] = [0, 0];
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

            gl::GenBuffers(2, ebos.as_mut_ptr() as *mut GLuint);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebos[0]);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size_of::<i32>() * indices.len()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as GLsizei,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            // texture attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * size_of::<f32>()) as GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        vaos
    };

    let (texture_one_id, texture_two_id) = {
        // Load the texture from memory
        let texture_data = ImageReader::open("./data/container.jpg")
            .unwrap()
            .decode()
            .unwrap();

        // Generate an opengl texture
        let texture_one_id = unsafe {
            let mut texture_id: GLuint = 0;
            gl::GenTextures(1, &mut texture_id as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // set the wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // Set the filtering parameters (minify and magnify)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                texture_data.width() as i32,
                texture_data.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                texture_data.as_bytes().as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            texture_id
        };

        // Load the texture from memory
        let texture_data = ImageReader::open("./data/awesomeface.png")
            .unwrap()
            .decode()
            .unwrap();

        // Generate an opengl texture
        let texture_two_id = unsafe {
            let mut texture_id: GLuint = 0;
            gl::GenTextures(1, &mut texture_id as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // set the wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            // Set the filtering parameters (minify and magnify)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                texture_data.width() as i32,
                texture_data.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture_data.as_bytes().as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            texture_id
        };

        (texture_one_id, texture_two_id)
    };

    // Set texture uniforms
    {
        shader_program.use_program();
        shader_program.set_int("texture1\0", 0);
        shader_program.set_int("texture2\0", 1);
    }

    // Projection transform
    let projection = make_projection_matrix(
        angle_to_rad(45.0),
        (width as f32) / (height as f32),
        0.1,
        100.0,
    );

    let cube_positions = [
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 2.0,
            y: 5.0,
            z: -15.0,
        },
        Vector3 {
            x: -1.5,
            y: -2.2,
            z: -2.5,
        },
        Vector3 {
            x: -3.8,
            y: -2.0,
            z: -12.3,
        },
        Vector3 {
            x: 2.4,
            y: -0.4,
            z: -3.5,
        },
        Vector3 {
            x: -1.7,
            y: 3.0,
            z: -7.5,
        },
        Vector3 {
            x: 1.3,
            y: -2.0,
            z: -2.5,
        },
        Vector3 {
            x: 1.5,
            y: 2.0,
            z: 2.5,
        },
        Vector3 {
            x: 1.5,
            y: 0.2,
            z: -1.5,
        },
        Vector3 {
            x: -1.3,
            y: 1.0,
            z: -1.5,
        },
    ];

    let mut camera = Camera::new();
    camera.position.z = 3.0;

    let time_start = Instant::now();
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events_receiver) {
            match event {
                WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        let millis_since = Instant::now().duration_since(time_start).as_millis() as f32;

        // Update Camera
        let view = {
            camera.target = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let orbit_radius = 20.0;
            let period = 10_000.0;
            camera.position = Vector3 {
                x: orbit_radius * (6.18 * millis_since / period).cos(),
                y: 0.0,
                z: orbit_radius * (6.18 * millis_since / period).sin(),
            };

            camera.view_matrix()
        };

        unsafe {
            // clear the color buffer
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_one_id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_two_id);

            // Render
            for (cube_index, cube_position) in cube_positions.iter().enumerate() {
                let transform = {
                    // World transforms
                    let transform =
                        Matrix4::translate(cube_position.x, cube_position.y, cube_position.z);

                    // Model transforms
                    let rotation = 20.0 * (cube_index as f32) + millis_since;
                    let period = 2000.0; // in ms
                    let transform = Matrix4::mult_mat4(
                        &transform,
                        &Matrix4::rotate_around_x(6.18 * rotation / period),
                    );

                    transform
                };
                shader_program.use_program();
                shader_program.set_mat4("model\0", &transform);
                shader_program.set_mat4("view\0", &view);
                shader_program.set_mat4("projection\0", &projection);
                gl::BindVertexArray(vaos[0]);
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const c_void);
            }
        }

        window.swap_buffers();
        glfw_data.poll_events();
    }
}

fn scene_two() {
    // Initialize GLFW and window
    let width = 800;
    let height = 600;
    let (mut glfw_data, mut window, events_receiver) = {
        let mut glfw_data = glfw::init_no_callbacks().unwrap();

        // Ask for version 3.3 and the core profile
        glfw_data.window_hint(WindowHint::ContextVersionMajor(3));
        glfw_data.window_hint(WindowHint::ContextVersionMinor(3));
        glfw_data.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        // Create a window object
        let (mut window, events_receiver) = glfw_data
            .create_window(width, height, "LearnOpenGL", WindowMode::Windowed)
            .unwrap();

        window.make_current();

        // Load opengl object pointers
        gl::load(|e| glfw_data.get_proc_address_raw(e).unwrap() as *const std::os::raw::c_void);

        // Set up viewport
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::DEPTH_TEST);
        }

        (glfw_data, window, events_receiver)
    };

    // TODO: register resize callback
    let shader_program = ShaderProgram::new(
        Path::new("./src/light_shader.vs"),
        Path::new("./src/light_shader.fs"),
    );
    let light_source_shader_program = ShaderProgram::new(
        Path::new("./src/light_shader.vs"),
        Path::new("./src/light_source_shader.fs"),
    );

    // Vertex input
    let cube_vertices: [f32; 216] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.5, 0.5, -0.5, 0.0,
        0.0, -1.0, 0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, -0.5, -0.5,
        -0.5, 0.0, 0.0, -1.0, -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.5,
        0.5, 0.5, 0.0, 0.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, -0.5,
        -0.5, 0.5, 0.0, 0.0, 1.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, -0.5, 0.5, -0.5, -1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, -0.5, -0.5, 0.5, -1.0,
        0.0, 0.0, -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 0.5, -0.5,
        1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.5, -0.5,
        0.5, 1.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 0.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.5,
        -0.5, -0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.5, -0.5, 0.5, 0.0, -1.0, 0.0,
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, -0.5, 0.5, -0.5, 0.0,
        1.0, 0.0, 0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.5, 0.5, 0.5, 0.0,
        1.0, 0.0, -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, -0.5, 0.5, -0.5, 0.0, 1.0, 0.0,
    ];
    let indices: [i32; 36] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    ];
    let vaos = {
        let mut vbos: [GLuint; 2] = [0, 0];
        let mut vaos: [GLuint; 2] = [0, 0];
        let mut ebos: [GLuint; 2] = [0, 0];
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
                (size_of::<f32>() * cube_vertices.len()) as isize,
                cube_vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW, // Data is set once and used many times
            );

            gl::GenBuffers(2, ebos.as_mut_ptr() as *mut GLuint);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebos[0]);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size_of::<i32>() * indices.len()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as GLsizei,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            // normals attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        unsafe {
            gl::BindVertexArray(vaos[1]);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebos[0]);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size_of::<i32>() * indices.len()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * size_of::<f32>() as GLsizei,
                0 as *const c_void,
            );
            gl::EnableVertexAttribArray(0);
        }
        vaos
    };

    // Projection transform
    let projection = make_projection_matrix(
        angle_to_rad(45.0),
        (width as f32) / (height as f32),
        0.1,
        100.0,
    );

    let light_pos = Vector3 {
        x: 1.5,
        y: 1.0,
        z: 3.0,
    };
    let cube_pos = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut camera = Camera::new();

    let time_start = Instant::now();
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events_receiver) {
            match event {
                WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        let millis_since = Instant::now().duration_since(time_start).as_millis() as f32;

        // Update Camera
        let view = {
            camera.target = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            camera.position = Vector3 {
                x: 2.5,
                y: 0.0,
                z: 5.0,
            };

            camera.view_matrix()
        };

        unsafe {
            // clear the color buffer
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Render

            // Other cube
            {
                let transform = {
                    // World transforms
                    let transform = Matrix4::translate(cube_pos.x, cube_pos.y, cube_pos.z);

                    // Model transforms
                    let period = 3000.0; // in ms
                    let rotation = 6.18 * millis_since / period;
                    let transform =
                        Matrix4::mult_mat4(&transform, &Matrix4::rotate_around_y(rotation));

                    transform
                };
                shader_program.use_program();
                shader_program.set_mat4("model\0", &transform);
                shader_program.set_mat4("view\0", &view);
                shader_program.set_mat4("projection\0", &projection);
                shader_program.set_vec3(
                    "objectColor\0",
                    &Vector3 {
                        x: 1.0,
                        y: 0.5,
                        z: 0.31,
                    },
                );
                shader_program.set_vec3(
                    "lightColor\0",
                    &Vector3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                );
                shader_program.set_vec3("lightPos\0", &light_pos);
                gl::BindVertexArray(vaos[0]);
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const c_void);
            }

            // Light source
            {
                let transform = {
                    // World transforms
                    let transform = Matrix4::translate(light_pos.x, light_pos.y, light_pos.z);

                    transform
                };
                light_source_shader_program.use_program();
                light_source_shader_program.set_mat4("model\0", &transform);
                light_source_shader_program.set_mat4("view\0", &view);
                light_source_shader_program.set_mat4("projection\0", &projection);
                gl::BindVertexArray(vaos[1]);
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const c_void);
            }
        }

        window.swap_buffers();
        glfw_data.poll_events();
    }
}

fn main() {
    // scene_one();
    scene_two();
}
