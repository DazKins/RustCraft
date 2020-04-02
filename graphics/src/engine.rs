use crate::window::Window;
use crate::game_state::GameState;
use std::time::{ Instant };

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32
}

pub struct Engine {
    window: Window
}

impl Engine {
    pub fn create(config: EngineConfig) -> Engine
    {
        let window = Window::new(config.window_width, config.window_height);
    
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
        const MILLIS_PER_TICK: f64 = 1000.0 / TICKS_PER_SECOND as f64;

        let mut last_frame_time = Instant::now();
        let mut delta = 0.0;

        while !self.window.was_close_requested {
            self.window.clear();
            self.window.process_events();

            let elapsed_time = Instant::now().checked_duration_since(last_frame_time);
            last_frame_time = Instant::now();
            
            delta = delta + (elapsed_time.unwrap().as_millis() as f64) / MILLIS_PER_TICK;

            while delta >= 1.0 {
                delta -= 1.0;
                game_state.tick();
            }

            game_state.render();

            self.window.close();

            self.window.update();
        }
    }
}
