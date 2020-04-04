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

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub struct GameStatePlaying {
    model: Model,
    shader: Shader
}

impl GameStatePlaying {
    pub fn new() -> GameStatePlaying {
        GameStatePlaying {
            model: Model::new(&VERTICES, &INDICES),
            shader: Shader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)
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
