use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;

struct Point {
    x: usize,
    y: usize,
    color: u32,
}

fn create_window(width: usize, height: usize) -> Window {
    let mut window = Window::new("Voronoi diagram", width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    return window;
}

fn pick_random_points(
    n: usize,
    width: usize,
    height: usize,
    radius: usize,
    rng: &mut ThreadRng,
) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(n);

    for _ in 0..n {
        let x = rng.gen_range(radius..width - radius);
        let y = rng.gen_range(radius..height - radius);
        let color = rng.gen_range(0..255);

        let point = Point { x, y, color };

        points.push(point);
    }

    points
}

fn draw_points(
    points: &Vec<Point>,
    buffer: &mut Vec<u32>,
    radius: usize,
    width: usize,
    height: usize,
) {
    for point in points {
        // Rows
        let base_index = point.y * width + point.x;
        for k in 0..radius {
            let index_of_first_pixel_in_this_row = base_index + width * k;
            buffer[index_of_first_pixel_in_this_row] = 0;

            // Columns
            for j in 0..radius {
                buffer[index_of_first_pixel_in_this_row + j] = 0;
            }
        }
    }
}

fn determin_pixel_aligance(
    points: &Vec<Point>,
    buffer: &mut Vec<u32>,
    radius: usize,
    width: usize,
    height: usize,
) {
    for pixel_index in 0..buffer.len() {

        for point in points {
            let point_index = point.y * width + point.x;

            let mut diff = 0;

            if pixel_index < point_index {
                diff = point_index - pixel_index;
            }
            if pixel_index >= point_index {
                diff = pixel_index - point_index;
            }

            // This is the slow approch, but I need to perform some experiments either way
        
        }
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut rng = rand::thread_rng();

    let mut window = create_window(WIDTH, HEIGHT);

    let mut buffer: Vec<u32> = vec![u32::MAX; WIDTH * HEIGHT];

    const n: usize = 10; //number of voronoi points
    const radius: usize = 10;

    let points = pick_random_points(n, WIDTH, HEIGHT, radius, &mut rng);

    draw_points(&points, &mut buffer, radius, WIDTH, HEIGHT);

    determin_pixel_aligance(&points, &mut buffer, radius, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
