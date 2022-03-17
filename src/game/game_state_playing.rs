use engine::input::InputState;
use engine::GameState;
use engine::RenderContext;

use crate::render::world::world::WorldRenderer;

use super::world::world::World;

pub struct GameStatePlaying {
    world: World,
    world_renderer: WorldRenderer,
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        GameStatePlaying {
            world: World::new(),
            world_renderer: WorldRenderer::new(),
        }
    }
}

impl GameState for GameStatePlaying {
    fn tick(&mut self, input_state: &InputState) {
        self.world.tick(input_state);
    }

    fn render(&mut self, render_context: &mut RenderContext) {
        self.world_renderer.render(render_context, &self.world);
    }
}
