mod game_states;

use engine::Engine;
use engine::EngineConfig;
use game_states::GameStatePlaying;

fn main() {
    let engine_config = EngineConfig {
        window_width: 800,
        window_height: 800,
    };

    let mut engine = Engine::new(engine_config);

    let mut game_state_playing = GameStatePlaying::new(&engine);

    engine.start(&mut game_state_playing);
}
