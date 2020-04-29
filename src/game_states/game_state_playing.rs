use engine::game_state::GameState;

use engine::shader::Shader;
use engine::model::Model;
use engine::model::ModelBuilder;

use cgmath::{ Matrix4, Vector3 };

pub struct GameStatePlaying {
    model: Model,
    shader: Shader
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        let mut model_builder = ModelBuilder::new();

        model_builder.set_xyz(Vector3::new(0.5, 0.5, 0.0)).push_vertex();
        model_builder.set_xyz(Vector3::new(0.5, -0.5, 0.0)).push_vertex();
        model_builder.set_xyz(Vector3::new(-0.5, 0.5, 0.0)).push_vertex();
        
        model_builder.set_xyz(Vector3::new(-0.5, -0.5, 0.0)).push_vertex();
        model_builder.set_xyz(Vector3::new(-0.5, 0.5, 0.0)).push_vertex();
        model_builder.set_xyz(Vector3::new(0.5, -0.5, 0.0)).push_vertex();

        let model = model_builder.build();

        GameStatePlaying {
            model,
            shader: Shader::new("shaders/vertex_shader.vs", "shaders/fragment_shader.fs")
        }
    }
}

impl GameState for GameStatePlaying {
    fn init(&mut self) {
        self.shader.bind();
        self.shader.set_uniform_mat4("transformMatrix", &Matrix4::from_scale(1.0_f32));
    }

    fn tick(&mut self) {

    }

    fn render(&mut self) {
        self.model.render();
    }
}
