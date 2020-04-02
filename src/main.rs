use graphics::Engine;
use game_states::GameStatePlaying;

mod game_states;

fn main() {
    let engine_config = graphics::EngineConfig {
        window_width: 800,
        window_height: 600
    };

    Engine::create(engine_config)
        .start(&mut GameStatePlaying::new());
}
