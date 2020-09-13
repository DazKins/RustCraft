use crate::window::Window;
use crate::input::InputState;
use crate::GameState;
use std::{time::{ Instant }, cell::RefCell};

use std::rc::Rc;

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32
}

pub struct Engine {
    window: Window
}

impl Engine {
    pub fn new(config: EngineConfig, input_state: Rc<RefCell<InputState>>) -> Engine
    {
        let window = Window::new(config.window_width, config.window_height, input_state);
    
        Engine {
            window
        }
    }

    pub fn start(&mut self, game_state: &mut dyn GameState) {
        game_state.init();

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

        while !self.window.was_close_requested {
            self.window.clear();
            self.window.process_events();

            let now = Instant::now();

            let elapsed_time = now.checked_duration_since(last_frame_time);
            last_frame_time = Instant::now();
            
            delta = delta + (elapsed_time.unwrap().as_nanos() as f64) / NANOS_PER_TICK;

            while delta >= 1.0 {
                delta -= 1.0;
                game_state.tick();
                tick_tracker = tick_tracker + 1;
            }

            if now.checked_duration_since(last_debug_print_time).unwrap().as_secs() >= 1
            {
                last_debug_print_time = now;

                println!("FPS: {}", frame_tracker);
                println!("UPS: {}", tick_tracker);

                frame_tracker = 0;
                tick_tracker = 0;
            }

            game_state.render();

            frame_tracker = frame_tracker + 1;

            self.window.close();

            self.window.update();
        }
    }
}
