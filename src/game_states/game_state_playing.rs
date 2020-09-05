use engine::game_state::GameState;

use engine::model::Model;
use engine::model::ModelBuilder;
use engine::texture::Texture;
use engine::render_context::RenderContext;

use cgmath::{ Vector3, Vector2 };

pub struct GameStatePlaying {
    model: Model,
    texture: Texture,
    render_context: RenderContext
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        let mut model_builder = ModelBuilder::new();

        model_builder
            .set_xyz(Vector3::new(0.5, 0.5, 0.0))
            .set_uv(Vector2::new(1.0, 1.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(0.5, -0.5, 0.0))
            .set_uv(Vector2::new(1.0, 0.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(-0.5, 0.5, 0.0))
            .set_uv(Vector2::new(0.0, 1.0))
            .push_vertex();
        
        model_builder
            .set_xyz(Vector3::new(-0.5, -0.5, 0.0))
            .set_uv(Vector2::new(0.0, 1.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(-0.5, 0.5, 0.0))
            .set_uv(Vector2::new(0.0, 0.0))
            .push_vertex();
        model_builder
            .set_xyz(Vector3::new(0.5, -0.5, 0.0))
            .set_uv(Vector2::new(1.0, 1.0))
            .push_vertex();

        let model = model_builder.build();

        let texture = Texture::new("container.jpg");
        let render_context = RenderContext::new();

        GameStatePlaying {
            model,
            render_context,
            texture
        }
    }
}

impl GameState for GameStatePlaying {
    fn init(&mut self) {

    }

    fn tick(&mut self) {

    }

    fn render(&mut self) {
        self.texture.bind();
        self.render_context.render(&self.model);
    }
}
