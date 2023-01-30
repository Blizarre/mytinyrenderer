use std::error::Error;
// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
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
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b, a: 255 }
    }

    pub fn black() -> Pixel {
        Self::new(0, 0, 0)
    }

    fn as_u8(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl<'a> Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        data.resize(width * height, Pixel::black());
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get(&'a mut self, x: usize, y: usize) -> &'a mut Pixel {
        if x >= self.width {
            panic!("Array out of bound"); // Will do for now
        }
        self.data.get_mut(y * self.width + x).unwrap()
    }

    pub fn write_to_file(self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let w = &mut BufWriter::new(file);

        let width_u32 = u32::try_from(self.width)?;
        let height_u32 = u32::try_from(self.height)?;
        let mut encoder = png::Encoder::new(w, width_u32, height_u32);
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
