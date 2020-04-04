use engine::game_state::GameState;

use engine::shader::Shader;
use engine::model::Model;

const VERTICES: [f32; 12] = [
    0.5,  0.5, 0.0, 
    0.5, -0.5, 0.0,
   -0.5,  0.5, 0.0,
   -0.5,  -0.5, 0.0
];

const INDICES: [i32; 6] = [
    0, 1, 2,
    3, 2, 1
];

pub struct GameStatePlaying {
    model: Model,
    shader: Shader
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        GameStatePlaying {
            model: Model::new(&VERTICES, &INDICES),
            shader: Shader::new("shaders/vertex_shader.vs", "shaders/fragment_shader.fs")
        }
    }
}

impl GameState for GameStatePlaying {
    fn init(&mut self) {
        
    }

    fn tick(&mut self) {

    }

    fn render(&mut self) {
        self.shader.bind();
        self.model.render();
    }
}
