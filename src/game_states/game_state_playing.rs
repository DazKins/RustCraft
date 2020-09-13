use engine::GameState;

use engine::model::Model;
use engine::model::ModelBuilder;
use engine::input::InputState;
use engine::input::Key;
use engine::Texture;
use engine::RenderContext;

use cgmath::{ Matrix4, Vector3, Vector2, Rad };

pub struct GameStatePlaying {
    model: Model,
    texture: Texture,
    t: f32
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

        GameStatePlaying {
            model,
            texture: Texture::new("container.jpg"),
            t: 0.0
        }
    }
}

impl GameState for GameStatePlaying {
    fn tick(&mut self, input_state: &InputState) {
        if input_state.is_key_pressed(Key::A) {
            self.t = self.t + 0.04;
        } else if input_state.is_key_pressed(Key::D) {
            self.t = self.t - 0.04;
        }
    }

    fn render(&mut self, render_context: &mut RenderContext) {
        self.texture.bind();
        render_context.get_matrix_stack().push();
        render_context.get_matrix_stack().transform(&Matrix4::from_angle_z(Rad(self.t)));
        render_context.render(&self.model);
        render_context.get_matrix_stack().pop();
    }
}
