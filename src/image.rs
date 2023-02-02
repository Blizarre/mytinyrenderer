use std::error::Error;
// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image {
    data: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    a: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }

    fn as_u8(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

pub const BLACK: Pixel = Pixel::new(0, 0, 0);

impl<'a> Image {
    pub fn new(width: u32, height: u32) -> Self {
        let mut data = Vec::new();
        data.resize((width * height) as usize, BLACK);
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get(&'a mut self, x: u32, y: u32) -> &'a mut Pixel {
        if x >= self.width {
            panic!("Invalid width (out of bound)"); // Will do for now
        }
        self.data.get_mut((y * self.width + x) as usize).unwrap()
    }

    pub fn write_to_file(self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        // Probably hugely inefficient, needs benchmarking
        let data = self
            .data
            .iter()
            .flat_map(|pixel| pixel.as_u8())
            .collect::<Vec<u8>>();
        writer.write_image_data(&data)?;
        Ok(())
    }
}
