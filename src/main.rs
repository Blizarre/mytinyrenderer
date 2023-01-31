use std::{f32::consts::PI, path::Path};

mod line_draw;
use line_draw::draw_line;

mod image;
use image::{Image, Pixel};

mod point;
use point::Point;

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
