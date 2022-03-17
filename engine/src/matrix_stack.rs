use cgmath::Matrix4;

pub struct MatrixStack {
    stack: Vec<Matrix4<f32>>,
}

impl MatrixStack {
    pub fn new() -> MatrixStack {
        let mut identity_stack = Vec::new();
        identity_stack.push(Matrix4::from_scale(1.0_f32));

        MatrixStack {
            stack: identity_stack,
        }
    }

    pub fn push(&mut self) {
        self.stack.push(*self.stack.last().unwrap());
    }

    pub fn transform(&mut self, matrix: &Matrix4<f32>) {
        let current_matrix = self.pop();
        let new_matrix = current_matrix * matrix;
        self.stack.push(new_matrix);
    }

    pub fn pop(&mut self) -> Matrix4<f32> {
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> &Matrix4<f32> {
        self.stack.last().unwrap()
    }
}
