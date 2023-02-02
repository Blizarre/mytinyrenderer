
use crate::{Pixel, Point, Image};

// From wikipedia: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm

// I use i32 which is < u32 so we halve the effective max size of the image
// Good enough for now... But need to keep that in mind

fn draw_line_low_nochecks(image: &mut Image, start: Point, end: Point, color: Pixel) {
    let dx = (end.x - start.x) as i32;
    let dy = (end.y - start.y) as i32;

    let (yi, dy) = if dy > 0 { (1, dy) } else { (-1, -dy) };

    let mut error = (2 * dy) - dx;
    let mut y = start.y as i32;

    for x in (start.x as i32)..(end.x as i32) {
        *image.get(x as u32, y as u32) = color;
        if error > 0 {
            y = y + yi;
            error = error + (2 * (dy - dx));
        } else {
            error = error + 2 * dy;
        }
    }
}

fn draw_line_high_nochecks(image: &mut Image, start: Point, end: Point, color: Pixel) {
    let dx = (end.x - start.x) as i32;
    let dy = (end.y - start.y) as i32;
    let (xi, dx) = if dx > 0 { (1, dx) } else { (-1, -dx) };

    let mut error = (2 * dx) - dy;
    let mut x = start.x as i32;

    for y in (start.y as i32)..(end.y as i32) {
        *image.get(x as u32, y as u32) = color;
        if error > 0 {
            x = x + xi;
            error = error + (2 * (dx - dy));
        } else {
            error = error + 2 * dx;
        }
    }
}

pub fn draw_line(image: &mut Image, p1: Point, p2: Point, color: Pixel) {
    // The line is fully within the image, so we do not need to do boundary checks
    if (p1.x as i32) >= 0 && (p1.x as i32) < image.width as i32 && (p2.y as i32) > 0 && (p2.y as i32) < image.height as i32 {
        if (p2.y - p1.y).abs() < (p2.x - p1.x).abs() {
            if p1.x > p2.x {
                draw_line_low_nochecks(image, p2, p1, color)
            } else {
                draw_line_low_nochecks(image, p1, p2, color)
            }
        } else {
            if p1.y > p2.y {
                draw_line_high_nochecks(image, p2, p1, color)
            } else {
                draw_line_high_nochecks(image, p1, p2, color)
            }
        }
    }
}
