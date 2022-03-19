mod game;
mod render;

use engine::Engine;
use engine::EngineConfig;
use game::GameStatePlaying;

fn main() {
    let engine_config = EngineConfig {
        window_width: 1280,
        window_height: 720,
    };

    Engine::new(engine_config).start(&mut GameStatePlaying::new());
}
