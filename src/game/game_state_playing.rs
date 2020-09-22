use engine::GameState;
use engine::input::InputState;
use engine::RenderContext;

use super::world::World;

pub struct GameStatePlaying {
    world: World
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        GameStatePlaying {
            world: World::new()
        }
    }
}

impl GameState for GameStatePlaying {
    fn tick(&mut self, input_state: &InputState) {
        self.world.tick(input_state);
    }

    fn render(&mut self, render_context: &mut RenderContext) {
        self.world.render(render_context);
    }
}
