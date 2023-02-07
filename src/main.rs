use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;

fn create_window(width: usize, height: usize) -> Window {
    let mut window = Window::new("Progress", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    return window;
}

fn main() {
    const WIDTH: usize  = 800;
    const HEIGHT: usize = 600;

    let mut rng = rand::thread_rng();

    let mut window = create_window(WIDTH, HEIGHT);

    let mut buffer: Vec<u32> = vec![240; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
