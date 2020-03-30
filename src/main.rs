use graphics::Window;
use graphics::Model;

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

fn main() {
    let mut window = Window::new(800, 600);
    let model = Model::new(&vertices, &indices);

    while !window.was_close_requested {
        window.clear();
        window.process_events();

        model.render();

        window.update();
    }

    window.close();
}
