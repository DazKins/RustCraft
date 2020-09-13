mod game_states;

use engine::Engine;
use engine::EngineConfig;
use game_states::GameStatePlaying;

use engine::input::InputState;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let engine_config = EngineConfig {
        window_width: 800,
        window_height: 800,
    };

    let input_state = Rc::new(RefCell::new(InputState::new()));

    let mut engine = Engine::new(engine_config, Rc::clone(&input_state));

    let mut game_state_playing = GameStatePlaying::new(Rc::clone(&input_state));

    engine.start(&mut game_state_playing);
}
