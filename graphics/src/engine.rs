use crate::window::Window;
use crate::game_state::GameState;

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
        while !self.window.was_close_requested {
            self.window.clear();
            self.window.process_events();

            game_state.tick();
            game_state.render();

            self.window.close();

            self.window.update();
        }
    }
}
