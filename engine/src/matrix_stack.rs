use cgmath::Matrix4;

pub struct MatrixStack {
    stack: Vec<Matrix4<f32>>
}

impl MatrixStack {
    pub fn new() -> MatrixStack {
        let mut identity_stack = Vec::new();
        identity_stack.push(Matrix4::from_scale(1.0_f32));

        MatrixStack {
            stack: identity_stack
        }
    }

    pub fn push(&mut self, matrix: Matrix4<f32>) -> &mut MatrixStack {
        let current_matrix = self.stack.last().unwrap();
        let new_matrix = current_matrix * matrix;
        self.stack.push(new_matrix);
        self
    }

    pub fn pop(&mut self) -> &mut MatrixStack {
        self.stack.pop().unwrap();
        self
    }

    pub fn peek(&self) -> &Matrix4<f32> {
        self.stack.last().unwrap()
    }
}
