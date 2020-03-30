use graphics::Window;
use graphics::Model;
use graphics::Shader;

const vertices: [f32; 12] = [
    0.5,  0.5, 0.0, 
    0.5, -0.5, 0.0,
   -0.5,  0.5, 0.0,
   -0.5,  -0.5, 0.0
];

const indices: [i32; 6] = [
    0, 1, 2,
    3, 2, 1
];

const vertex_shader_source: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    let mut window = Window::new(800, 600);
    let model = Model::new(&vertices, &indices);
    let shader = Shader::new(vertex_shader_source, fragment_shader_source);

    while !window.was_close_requested {
        window.clear();
        window.process_events();

        shader.bind();
        model.render();

        window.update();
    }

    window.close();
}
