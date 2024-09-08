mod point;
use std::{env, f32::consts::PI, path::Path};

use image::{imageops::FilterType, GenericImageView, GrayImage, ImageBuffer, ImageReader, Pixel};
use point::Point;

const STRING_ALPHA: u8 = 25;
const SIZE: u32 = 500;

fn main() {
    let args: Vec<_> = env::args().collect();
    let npoints = if args.len() >= 2 {
        args[2]
            .parse::<u32>()
            .expect("Couldn't parse number of points")
    } else {
        100
    };

    let sequence = generate_string_sequence(Path::new(&args[1]), npoints);
    println!("{:?}", sequence);

    let points = generate_points(npoints, SIZE);
    let mut output = GrayImage::new(SIZE, SIZE);

    let mut prev_point = sequence[0];
    for point in &sequence[1..] {
        for p in generate_line(prev_point, point) {
            output.
        }
    }
}

fn generate_string_sequence(filepath: &Path, npoints: u32) -> Vec<u32> {
    let mut sequence = Vec::with_capacity(npoints as usize);
    sequence.push(0);

    let points = generate_points(npoints, SIZE);
    let mut target = vec![vec![0u8; SIZE as usize + 1]; SIZE as usize + 1];
    let pixels = ImageReader::open(filepath)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .resize(SIZE, SIZE, FilterType::Gaussian);

    for (x, y, p) in pixels.grayscale().pixels() {
        target[y as usize][x as usize] = p.to_luma().0.first().unwrap() / STRING_ALPHA;
    }

    for start in &points {
        let lines: Vec<_> = points
            .iter()
            .map(|end| generate_line(*start, *end))
            .collect();
        let improvements = lines.iter().enumerate().map(|(i, line)| {
            if !sequence.contains(&(i as u32)) {
                calc_improvement(line, &target[..])
            } else {
                0
            }
        });

        let mut maximum = 0;
        let mut max_idx = 0;

        for (i, improvement) in improvements.enumerate() {
            if improvement > maximum {
                maximum = improvement;
                max_idx = i;
            }
        }

        sequence.push(max_idx as u32);
        for point in &lines[max_idx] {
            target[point.y as usize][point.x as usize] =
                target[point.y as usize][point.x as usize].saturating_sub(1);
        }
    }

    println!(
        "error value: {}",
        target
            .iter()
            .flatten()
            .map(|&val| val as u64 * val as u64)
            .sum::<u64>()
    );
    sequence
}

fn calc_improvement(line: &[Point<u32>], target: &[Vec<u8>]) -> u32 {
    line.iter()
        .filter(|p| target[p.y as usize][p.x as usize] != 0)
        .count()
        .try_into()
        .unwrap()
}

fn generate_line(p0: Point<u32>, p1: Point<u32>) -> Vec<Point<u32>> {
    let (dx, dy) = (p1.x.abs_diff(p0.x) as i32, -(p1.y.abs_diff(p0.y) as i32));

    if dx == 0 && dy == 0 {
        return Vec::new();
    }

    let mut points = Vec::with_capacity(dx.max(dy) as usize);
    let (sx, sy) = (
        if p0.x < p1.x { 1 } else { -1 },
        if p0.y < p1.y { 1 } else { -1 },
    );
    let mut error = dx + dy;
    let Point { mut x, mut y } = p0;

    loop {
        points.push(Point::new(x, y));

        if x == p1.x && y == p1.y {
            break;
        }

        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x = (x as i32 + sx) as u32;
        }
        if e2 <= dx {
            error += dx;
            y = (y as i32 + sy) as u32;
        }
    }
    points
}

fn generate_points(points: u32, size: u32) -> Vec<Point<u32>> {
    (0..points)
        .map(|i| {
            let angle = 2.0 * PI * points as f32 / i as f32;
            let (angle_sin, angle_cos) = angle.sin_cos();
            let (x, y) = (
                (size + (angle_cos * size as f32) as u32) / 2,
                (size + (angle_sin * size as f32) as u32) / 2,
            );
            Point::new(x, y)
        })
        .collect()
}
