use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::{f32::consts::PI};

use mytinyrenderer::line_draw::draw_line;
use mytinyrenderer::image::{Image, Pixel};
use mytinyrenderer::point::Point;

fn line_draw_all_inside(image: &mut Image) {
    for d in 0..100 {
        let color = Pixel::new(2 * d as u8, 255 - 2 * d as u8, 255 - d as u8);
        let d = ((d as f32) / 100.) * (2. * PI);

        draw_line(
            image,
            Point::new(320., 240.),
            Point::new(300. * d.cos() + 320., 200. * d.sin() + 240.),
            color,
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut image = Image::new(640, 480);
    c.bench_function("line_draw_all_inside", |b| b.iter(|| line_draw_all_inside(black_box(&mut image))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
