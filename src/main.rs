
use graphics::Window;
use graphics::Model;

fn main() {
    let mut window = Window::new(800, 600);
    let model = Model::new();

    while !window.was_close_requested {
        window.clear();
        window.process_events();

        model.render();

        window.update();
    }

    window.close();
}
