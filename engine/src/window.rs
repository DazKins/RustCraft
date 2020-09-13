use glfw::{ Context, Action };

use std::{sync::mpsc::Receiver, cell::RefCell};
use std::rc::Rc;

use crate::input::InputState;

use crate::input::glfw_key_to_engine_key;

pub struct Window {
    pub was_close_requested: bool,
    glfw: glfw::Glfw,
    glfw_window: glfw::Window,
    event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    input_state: Rc<RefCell<InputState>>
}

impl Window {
    pub fn new(width: u32, height: u32 , input_state: Rc<RefCell<InputState>>) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    
        let (mut glfw_window, event_receiver) = 
            glfw.create_window(width, height, "Rust GLFW", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
    
        glfw_window.make_current();
        glfw_window.set_key_polling(true);
        glfw_window.set_framebuffer_size_polling(true);
        gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);
    
        Window {
            was_close_requested: false,
            glfw,
            glfw_window,
            event_receiver,
            input_state
        }
    }

    pub fn process_events (&mut self) {
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => self.was_close_requested = true,
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match action {
                        
                        Action::Press => self.input_state.borrow_mut().set_pressed(glfw_key_to_engine_key(key)),
                        Action::Release => self.input_state.borrow_mut().set_released(glfw_key_to_engine_key(key)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn close(&mut self) {
        self.glfw_window.set_should_close(true)
    }

    pub fn update(&mut self) {
        self.glfw_window.swap_buffers();
        self.glfw.poll_events();
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
    }
}
