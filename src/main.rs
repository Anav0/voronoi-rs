use clap::{Parser, ValueEnum};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::prelude::*;
use std::time::SystemTime;

struct Point {
    x: usize,
    y: usize,
    color: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about ="Program for displaying voronoi diagrams. Use '{' and '}' to change distance calculation function. '+' and '-' to change how many points are drawn", long_about = None)]
struct Params {
    #[arg(short, help = "Number of seeds", default_value = "10")]
    n: usize,

    #[arg(long, help = "Image width in pixels", default_value_t = 800)]
    width: usize,

    #[arg(long, help = "Image height in pixels", default_value_t = 600)]
    height: usize,

    #[arg(short, help = "Which distance calculation function to use")]
    distance: DistanceFn,
}

#[derive(ValueEnum, Clone, Debug)]
enum DistanceFn {
    Euclidian,
    Manhattan,
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

fn draw_points(points: &Vec<Point>, buffer: &mut Vec<u32>, radius: usize, width: usize) {
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

type distance_fn = dyn Fn(isize, isize, isize, isize) -> isize;

fn euclidian_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn determine_pixel_allegiance(
    points: &Vec<Point>,
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    distance_type: &DistanceFn,
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

                let distance = match distance_type {
                    DistanceFn::Euclidian => euclidian_distance(x1, y1, x2, y2),
                    DistanceFn::Manhattan => manhattan_distance(x1, y1, x2, y2),
                };

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

fn recompute(
    params: &Params,
    buffer: &mut Vec<u32>,
    colors: &Vec<u32>,
    radius: usize,
    distance: &DistanceFn,
    rng: &mut ThreadRng,
) -> Vec<Point> {
    let points = pick_random_points(params.n, params.width, params.height, &colors, radius, rng);

    determine_pixel_allegiance(&points, buffer, params.width, params.height, distance);

    draw_points(&points, buffer, radius, params.width);

    points
}

fn lerp(start: f64, end: f64, dt: f64) -> f64 {
    start * (1.0 - dt) + end * dt
}

fn draw_gradient(dt: f64, buffer: &mut Vec<u32>, width: usize, height: usize, step_size: usize) {
    let mut color: u32 = 0;

    for y in 0..height {
        if y % step_size == 0 {
            color += 1;
        }
        for x in 0..width {
            let index = y * width + x;
            buffer[index] = color;
        }
    }
}

fn main() {
    let mut params = Params::parse();

    let pallette: Vec<&str> = vec![
        "57ab5a", "eac55f", "f69d50", "f47068", "b083f0", "6cb6ff", "648c84", "24205c", "eda63d",
        "f2a19d", "890b3b", "87ad2f", "afc6f2", "cbd1c6", "001231", "0079b4", "b7b8a3", "e2affe",
    ];
    let mut colors: Vec<u32> = vec![];

    for hex in pallette {
        let color = u32::from_str_radix(&hex, 16).unwrap();
        colors.push(color);
    }

    let mut rng = rand::thread_rng();

    let mut window = create_window(params.width, params.height);

    let mut buffer: Vec<u32> = vec![u32::MAX; params.width * params.height];

    let mut points: Vec<Point> = Vec::with_capacity(params.n);

    const RADIUS: usize = 5;

    let mut dt: f64 = 0.0;
    let mut last = SystemTime::now();

    points = recompute(
        &params,
        &mut buffer,
        &colors,
        RADIUS,
        &params.distance,
        &mut rng,
    );

    let mut step_size = 10;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = SystemTime::now();
        dt = now.duration_since(last).unwrap().as_secs_f64() / 1000.0;
        last = now;

        //draw_gradient(dt, &mut buffer, params.width, params.height, step_size);

        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::L => {
                    step_size += 1;
                },
                Key::J => {
                    if step_size > 1 {
                        step_size -= 1;
                    }
                }
                Key::R => {
                    points = recompute(
                        &params,
                        &mut buffer,
                        &colors,
                        RADIUS,
                        &params.distance,
                        &mut rng,
                    );
                }
                Key::LeftBracket => {
                    determine_pixel_allegiance(
                        &points,
                        &mut buffer,
                        params.width,
                        params.height,
                        &DistanceFn::Euclidian,
                    );
                    draw_points(&points, &mut buffer, RADIUS, params.width);
                }
                Key::RightBracket => {
                    determine_pixel_allegiance(
                        &points,
                        &mut buffer,
                        params.width,
                        params.height,
                        &DistanceFn::Manhattan,
                    );
                    draw_points(&points, &mut buffer, RADIUS, params.width);
                }
                Key::NumPadPlus => {
                    if params.n < 100 {
                        params.n += 1;
                    }

                    points = recompute(
                        &params,
                        &mut buffer,
                        &colors,
                        RADIUS,
                        &params.distance,
                        &mut rng,
                    );
                }
                Key::NumPadMinus => {
                    if params.n > 1 {
                        params.n -= 1;
                    }

                    points = recompute(
                        &params,
                        &mut buffer,
                        &colors,
                        RADIUS,
                        &params.distance,
                        &mut rng,
                    );
                }
                _ => (),
            });

        window
            .update_with_buffer(&buffer, params.width, params.height)
            .unwrap();
    }
}
