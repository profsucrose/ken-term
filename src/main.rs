mod models;
mod opengl;

use std::time::Instant;

use cgmath::vec3;
use glfw::{Action, Context, Key, WindowEvent};
use gl::types::*;
use models::console::Console;
use opengl::text_renderer::TextRenderer;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 600;

fn main() {
    // wrap program in helper
    // for unsafe block w/o indentation
    unsafe { start(); }
}

unsafe fn start() {
    // glfw: initialize
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)); 

    // glfw window creation
    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "BitSnake", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_opacity(0.9);
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    window.set_title("KenTerm");

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut instant = Instant::now();

    let mut text_renderer = TextRenderer::new(
        WIDTH, 
        HEIGHT, 
        "assets/font/DOS VGA.ttf", 
        "assets/shaders/text_vertex.vert", 
        "assets/shaders/text_fragment.frag",
    );

    let mut console = Console::new(text_renderer);

    // target fps
    let target_fps = 120.0;

    // render loop
    while !window.should_close() {
        instant = Instant::now();

        // events
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    console.update_screen_size(width as u32, height as u32);
                    gl::Viewport(0, 0, width, height);
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                WindowEvent::Key(Key::LeftShift, _, Action::Press, _) => console.shift(),
                WindowEvent::Key(Key::LeftShift, _, Action::Release, _) => console.unshift(),
                WindowEvent::Key(key, _, Action::Press, _) => {
                    console.handle_key(key);
                },
                _ => ()
            }
        }

        // clear buffers
        gl::ClearColor(0.0, 0.0, 0.0, 0.8);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); 
        
        console.draw_lines();

        window.swap_buffers();
        glfw.poll_events();

        // hang thread for target FPS
        while (instant.elapsed().as_millis() as f32) < (1000.0 / target_fps) {}
    }
}