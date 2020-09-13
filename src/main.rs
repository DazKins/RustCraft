mod game_states;

use engine::Engine;
use engine::EngineConfig;
use game_states::GameStatePlaying;

fn main() {
    let engine_config = EngineConfig {
        window_width: 800,
        window_height: 800,
    };

    Engine::new(engine_config)
        .start(&mut GameStatePlaying::new());
}
