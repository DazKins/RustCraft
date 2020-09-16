use engine::GameState;

use engine::model::Model;
use engine::model::ModelBuilder;
use engine::input::InputState;
use engine::Texture;
use engine::RenderContext;

use cgmath::{ Vector3, Vector2 };

pub struct GameStatePlaying {
    model: Model,
    texture: Texture
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        let mut model_builder = ModelBuilder::new();

        model_builder
            .set_xyz(Vector3::new(0.5, 0.5, -1.0))
            .set_uv(Vector2::new(1.0, 1.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(0.5, -0.5, -1.0))
            .set_uv(Vector2::new(1.0, 0.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(-0.5, 0.5, -1.0))
            .set_uv(Vector2::new(0.0, 1.0))
            .push_vertex();
        
        model_builder
            .set_xyz(Vector3::new(-0.5, -0.5, -1.0))
            .set_uv(Vector2::new(0.0, 1.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(-0.5, 0.5, -1.0))
            .set_uv(Vector2::new(0.0, 0.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(0.5, -0.5, -1.0))
            .set_uv(Vector2::new(1.0, 1.0))
            .push_vertex();

        let model = model_builder.build();

        GameStatePlaying {
            model,
            texture: Texture::new("container.jpg")
        }
    }
}

impl GameState for GameStatePlaying {
    fn tick(&mut self, input_state: &InputState) {
    }

    fn render(&mut self, render_context: &mut RenderContext) {
        self.texture.bind();
        render_context.render(&self.model);
    }
}
