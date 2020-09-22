use cgmath::{Vector2, Vector3};
use engine::{RenderContext, Texture, input::InputState, model::Model, model::ModelBuilder};

pub struct World {
    model: Model,
    texture: Texture
}

impl World {
    pub fn new() -> Self {
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

        World {
            model,
            texture: Texture::new("container.jpg"),
        }
    }

    pub fn tick(&mut self, input_state: &InputState) {
        
    }

    pub fn render(&self, render_context: &mut RenderContext) {
        self.texture.bind();
        render_context.render(&self.model);
    }
}
