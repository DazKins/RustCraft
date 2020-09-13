use engine::GameState;

use engine::model::Model;
use engine::model::ModelBuilder;
use engine::Texture;
use engine::RenderContext;
use engine::input::InputState;
use engine::input::Key;

use cgmath::{ Matrix4, Vector3, Vector2, Rad };

use std::{rc::Rc, cell::RefCell};

pub struct GameStatePlaying {
    model: Model,
    texture: Texture,
    render_context: RenderContext,
    input_state: Rc<RefCell<InputState>>,
    t: f32
}

impl GameStatePlaying {
    pub fn new(input_state: Rc<RefCell<InputState>>) -> GameStatePlaying {
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
            texture,
            input_state,
            t: 0.0
        }
    }
}

impl GameState for GameStatePlaying {
    fn init(&mut self) {

    }

    fn tick(&mut self) {
        self.t = self.t + 0.01;

        if self.input_state.borrow().is_key_pressed(Key::J) {
            println!("SUCCESS!!!");
        }
    }

    fn render(&mut self) {
        self.texture.bind();
        self.render_context.get_matrix_stack().push();
        self.render_context.get_matrix_stack().transform(&Matrix4::from_angle_z(Rad(self.t)));
        self.render_context.render(&self.model);
        self.render_context.get_matrix_stack().pop();
    }
}
