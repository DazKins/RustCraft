use crate::matrix_stack::MatrixStack;
use crate::model::Model;
use crate::shader::Shader;

pub struct RenderContext {
    matrix_stack: MatrixStack,
    shader: Shader
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            matrix_stack: MatrixStack::new(),
            shader: Shader::new("shaders/vertex_shader.vs", "shaders/fragment_shader.fs")
        }
    }

    pub fn render(&self, model: &Model) {
        let matrix = self.matrix_stack.peek();
        self.shader.bind();
        self.shader.set_uniform_mat4("transformMatrix", matrix);
        model.render();
    }

    pub fn get_matrix_stack(&mut self) -> &mut MatrixStack {
        return &mut self.matrix_stack;
    }
}
