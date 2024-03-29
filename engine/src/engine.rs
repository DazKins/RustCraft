use crate::GameState;
use crate::RenderContext;
use crate::{Camera, Window};
use std::time::Instant;

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32,
}

pub struct Engine {
    window: Window,
    render_context: RenderContext,
    camera: Camera,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Engine {
        Engine {
            window: Window::new(config.window_width, config.window_height),
            render_context: RenderContext::new(),
            camera: Camera::new(90.0, config.window_width as f32 / config.window_height as f32, 0.01, 1000.0),
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

            if now
                .checked_duration_since(last_debug_print_time)
                .unwrap()
                .as_secs()
                >= 1
            {
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

        self.camera.set_position(game_state.get_camera_position());
        self.camera.set_rotation(game_state.get_camera_rotation());

        game_state.tick(self.window.get_input_state());
    }

    fn render(&mut self, game_state: &mut dyn GameState) {
        self.render_context.get_matrix_stack().push();
        self.render_context
            .get_matrix_stack()
            .transform(&self.camera.get_transform_matrix());
        game_state.render(&mut self.render_context);
        self.render_context.get_matrix_stack().pop();
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        return &mut self.camera
    }
}
