use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mytinyrenderer::line_draw::draw_line;
use mytinyrenderer::image::{Image, Pixel};
use mytinyrenderer::point::Point;

fn line_draw_all_inside(image: &mut Image) {
    for d in 0..200 {
        let color = Pixel::new(d as u8, 255 - d as u8, d % 64 + 128 as u8);
        let df = d as f32;
        draw_line(
            image,
            Point::new(100. + 5. * df, 100. + 4. * df),
            Point::new(1900. - 3. * df, 200. + 6. * df),
            color,
        );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut image = Image::new(1920, 1080);
    c.bench_function("line_draw_all_inside", |b| b.iter(|| line_draw_all_inside(black_box(&mut image))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
