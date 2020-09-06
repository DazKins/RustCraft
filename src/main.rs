use engine::Engine;
use game_states::GameStatePlaying;
use engine::EngineConfig;

mod game_states;

fn main() {
    let engine_config = EngineConfig {
        window_width: 800,
        window_height: 800
    };

    Engine::create(engine_config)
        .start(&mut GameStatePlaying::new());
}
