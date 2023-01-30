use std::path::Path;

mod image;

fn main() {
    let mut image = image::Image::new(640, 480);
    for x in 0..255 {
        *image.get(x, 5) = image::Pixel::new(255, 0, 0);
    }
    image.write_to_file(Path::new("image.png")).unwrap();
}
