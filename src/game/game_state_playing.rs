use cgmath::Vector3;
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

    fn get_camera_position(&self) -> Vector3<f32> {
        self.world.get_player().get_position()
    }

    fn get_camera_rotation(&self) -> Vector3<f32> {
        self.world.get_player().get_rotation()
    }
}
