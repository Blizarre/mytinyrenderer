// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 10, 10);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    // Generating a string of red/black pixels to cover the whole image
    let data = [255, 0, 0, 255, 0, 0, 0, 255].iter().cloned().cycle().take(10*10*4);
    let data:  Vec<u8> =  Vec::from_iter(data);
    writer.write_image_data(&data).unwrap(); // Save
}
