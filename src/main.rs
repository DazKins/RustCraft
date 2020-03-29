
use graphics::Window;

fn main() {
    let mut window = Window::new(800, 600);

    while !window.was_close_requested {
        window.process_events();

        window.update();
    }

    window.close();
}
