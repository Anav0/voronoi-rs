use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use structopt::StructOpt;

struct Point {
    x: usize,
    y: usize,
    color: u32,
}

#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(short = "n", long, help = "Number of seeds", default_value = "10")]
    n: usize,

    #[structopt(short = "w", help = "Image width in pixels", default_value = "800")]
    width: usize,

    #[structopt(short = "h", help = "Image height in pixels", default_value = "600")]
    height: usize,
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
    colors: &Vec<u32>,
    radius: usize,
    rng: &mut ThreadRng,
) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(n);

    for i in 0..n {
        let x = rng.gen_range(radius..width - radius);
        let y = rng.gen_range(radius..height - radius);
        let color = colors[i % colors.len()];

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

fn euclidian_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}

fn determin_pixel_aligance(
    points: &Vec<Point>,
    buffer: &mut Vec<u32>,
    radius: usize,
    width: usize,
    height: usize,
) {
    for x in 0..width {
        for y in 0..height {
            let mut closest_point_index = 0;
            let mut closest_point_diff = isize::MAX;

            let mut index = 0;
            for point in points {
                let x1 = point.x.try_into().unwrap();
                let y1 = point.y.try_into().unwrap();
                let x2 = x.try_into().unwrap();
                let y2 = y.try_into().unwrap();

                let distance = euclidian_distance(x1, y1, x2, y2);
                if distance < closest_point_diff {
                    closest_point_index = index;
                    closest_point_diff = distance;
                }
                index += 1;
            }
            buffer[y * width + x] = points[closest_point_index].color;
        }
    }
}

fn main() {
    let params = Params::from_args();

    let pallette: Vec<&str> = vec![
        "57ab5a", "eac55f", "f69d50", "f47068", "b083f0", "6cb6ff", "648c84", "24205c", "eda63d",
        "f2a19d", "890b3b", "87ad2f", "afc6f2", "cbd1c6", "001231", "0079b4", "b7b8a3", "e2affe"
    ];
    let mut colors: Vec<u32> = vec![];

    for hex in pallette {
        let color = u32::from_str_radix(&hex, 16).unwrap();
        colors.push(color);
    }

    let mut rng = rand::thread_rng();

    let mut window = create_window(params.width, params.height);

    let mut buffer: Vec<u32> = vec![u32::MAX; params.width * params.height];

    const radius: usize = 5;

    let points = pick_random_points(
        params.n,
        params.width,
        params.height,
        &colors,
        radius,
        &mut rng,
    );

    determin_pixel_aligance(&points, &mut buffer, radius, params.width, params.height);

    draw_points(&points, &mut buffer, radius, params.width, params.height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, params.width, params.height)
            .unwrap();
    }
}
