use crate::{Window, Camera};
use crate::input::InputState;
use crate::GameState;
use crate::RenderContext;
use std::{time::Instant, cell::RefCell, rc::Rc};

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32
}

pub struct Engine {
    window: Window,
    input_state: Rc<RefCell<InputState>>,
    render_context: RenderContext,
    camera: Camera
}

impl Engine {
    pub fn new(config: EngineConfig) -> Engine
    {
        let input_state = Rc::new(RefCell::new(InputState::new()));

        Engine {
            window: Window::new(config.window_width, config.window_height, Rc::clone(&input_state)),
            input_state: Rc::clone(&input_state),
            render_context: RenderContext::new(),
            camera: Camera::new(90.0, 1.0, 0.01, 1000.0)
        }
    }

    pub fn start(&mut self, game_state: &mut dyn GameState) {
        self.run_loop(game_state);

        self.window.close();
    }
    
    fn run_loop(&mut self, game_state: &mut dyn GameState) {
        const TICKS_PER_SECOND: i32 = 60;
        const NANOS_PER_TICK: f64 = 1000000000.0 / TICKS_PER_SECOND as f64;

        let mut frame_tracker = 0;
        let mut tick_tracker = 0;

        let mut last_frame_time = Instant::now();
        let mut delta = 0.0;

        let mut last_debug_print_time = Instant::now();

        while !self.window.should_close {
            self.window.clear();

            let now = Instant::now();

            let elapsed_time = now.checked_duration_since(last_frame_time);
            last_frame_time = Instant::now();
            
            delta += (elapsed_time.unwrap().as_nanos() as f64) / NANOS_PER_TICK;

            while delta >= 1.0 {
                delta -= 1.0;

                self.tick(game_state);

                tick_tracker += 1;
            }

            if now.checked_duration_since(last_debug_print_time).unwrap().as_secs() >= 1 {
                last_debug_print_time = now;

                println!("FPS: {}", frame_tracker);
                println!("UPS: {}", tick_tracker);

                frame_tracker = 0;
                tick_tracker = 0;
            }

            self.render(game_state);

            frame_tracker += 1;

            self.window.update();
        }
    }

    fn tick(&mut self, game_state: &mut dyn GameState) {
        self.window.tick();
        
        self.window.lock_mouse();
        self.camera.tick(&self.input_state.borrow());
        game_state.tick(&self.input_state.borrow());
    }

    fn render(&mut self, game_state: &mut dyn GameState) {
        self.render_context.get_matrix_stack().push();
        self.render_context.get_matrix_stack().transform(&self.camera.get_transform_matrix());
        game_state.render(&mut self.render_context);
        self.render_context.get_matrix_stack().pop();
    }
}
