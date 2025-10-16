use glad_gl::gl;
use glfw::{self, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

fn main() {
    // Initialize GLFW
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

    gl::load(|e| glfw_data.get_proc_address_raw(e).unwrap() as *const std::os::raw::c_void);

    unsafe {gl::Viewport(0, 0, 800, 600);}

    // TODO: register resize callback

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
