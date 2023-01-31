use std::{f32::consts::PI, path::Path};

mod image;
use image::{Image, Pixel};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

// From wikipedia: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm

fn draw_line_low(image: &mut Image, start: Point, end: Point, color: Pixel) {
    //println!("Drawing from {:?} to {:?}", start, end);
    let dx = (end.x - start.x) as i32;
    let dy = (end.y - start.y) as i32;

    let (yi, dy) = if dy > 0 { (1, dy) } else { (-1, -dy) };

    let mut error = (2 * dy) - dx;
    let mut y = start.y as i32;

    for x in (start.x as usize)..(end.x as usize) {
        *image.get(x, y as usize) = color;
        if error > 0 {
            y = y + yi;
            error = error + (2 * (dy - dx));
        } else {
            error = error + 2 * dy;
        }
    }
}

fn draw_line_high(image: &mut Image, start: Point, end: Point, color: Pixel) {
    let dx = (end.x - start.x) as i32;
    let dy = (end.y - start.y) as i32;
    let (xi, dx) = if dx > 0 { (1, dx) } else { (-1, -dx) };

    let mut error = (2 * dx) - dy;
    let mut x = start.x as i32;

    for y in (start.y as usize)..(end.y as usize) {
        *image.get(x as usize, y) = color;
        if error > 0 {
            x = x + xi;
            error = error + (2 * (dx - dy));
        } else {
            error = error + 2 * dx;
        }
    }
}

fn draw_line(image: &mut Image, p1: Point, p2: Point, color: Pixel) {
    if (p2.y - p1.y).abs() < (p2.x - p1.x).abs() {
        if p1.x > p2.x {
            draw_line_low(image, p2, p1, color)
        } else {
            draw_line_low(image, p1, p2, color)
        }
    } else {
        if p1.y > p2.y {
            draw_line_high(image, p2, p1, color)
        } else {
            draw_line_high(image, p1, p2, color)
        }
    }
}

fn main() {
    let mut image = Image::new(640, 480);
    for d in 0..100 {
        let color = Pixel::new(2 * d as u8, 255 - 2 * d as u8, 255 - d as u8);
        let d = ((d as f32) / 100.) * (2. * PI);

        draw_line(
            &mut image,
            Point::new(320., 240.),
            Point::new(300. * d.cos() + 320., 200. * d.sin() + 240.),
            color,
        );
    }
    image.write_to_file(Path::new("image.png")).unwrap();
}
