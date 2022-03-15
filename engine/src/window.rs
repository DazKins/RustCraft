use cgmath::Vector2;
use glfw::{ Context, Action , CursorMode};

use std::sync::mpsc::Receiver;

use super::input::InputState;

use super::input::glfw_key_to_engine_key;

pub struct Window {
    pub should_close: bool,
    width: u32,
    height: u32,
    glfw: glfw::Glfw,
    glfw_window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    input_state: InputState,
    mouse_locked: bool
}

impl Window {
    pub fn get_input_state(&self) -> &InputState {
        &self.input_state
    }

    pub fn new(width: u32, height: u32) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut glfw_window, event_receiver) =
            glfw.create_window(width, height, "Rust GLFW", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        glfw_window.make_current();
        glfw_window.set_all_polling(true);

        gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        Window {
            should_close: false,
            width,
            height,
            glfw,
            glfw_window,
            event_receiver,
            input_state: InputState::new(),
            mouse_locked: false
        }
    }

    pub fn tick (&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::Close | glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => self.should_close = true,
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    self.input_state.set_mouse_position(&Vector2::new(x as f32, y as f32))
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match action {
                        Action::Press => self.input_state.set_pressed(glfw_key_to_engine_key(key)),
                        Action::Release => self.input_state.set_released(glfw_key_to_engine_key(key)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if self.mouse_locked {
            let centre = self.get_centre();
            let mouse_position = self.input_state.get_mouse_position();
            self.input_state.set_mouse_speed(&(mouse_position - centre));
            self.set_cursor_pos(&self.get_centre())
        }
    }

    pub fn lock_mouse(&mut self) {
        self.mouse_locked = true;
        self.glfw_window.set_cursor_mode(CursorMode::Hidden);
    }

    pub fn unlock_mouse(&mut self) {
        self.mouse_locked = false;
        self.glfw_window.set_cursor_mode(CursorMode::Normal);
    }

    pub fn close(&mut self) {
        self.glfw_window.set_should_close(true)
    }

    pub fn update(&mut self) {
        self.glfw_window.swap_buffers();
    }

    pub fn set_cursor_pos(&mut self, position: &Vector2<f32>) {
        self.glfw_window.set_cursor_pos(position.x as f64, position.y as f64)
    }

    pub fn get_centre(&self) -> Vector2<f32> {
        return Vector2::new((self.width / 2) as f32, (self.height / 2) as f32)
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
    }
}
