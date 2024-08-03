use std::{env, fs, io};

use stb::image::Channels;

const MAGENTA: [u8; 3] = [255, 0, 255];

#[derive(Default)]
pub struct Image {
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
    // Linear floating point pixel data
    // fdata: Vec<f32>,
    // Linear 8-bit pixel data
    bdata: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let file_path = match env::var("IMAGE_DIR") {
            Ok(dir) => format!("{dir}/{filename}"),
            Err(_) => format!("images/{filename}"),
        };

        let file = match fs::File::open(&file_path) {
            Ok(f) => f,
            Err(_) => {
                println!("Could not open '{file_path}'");
                return Self::default();
            }
        };

        let mut reader = io::BufReader::new(file);
        Self::load(&mut reader)
    }

    pub fn load(reader: &mut io::BufReader<fs::File>) -> Self {
        let (info, data) = stb::image::stbi_loadf_from_reader(reader, Channels::Rgb)
            .expect("Should be able to load file with stb_image");

        // RGB u8
        let bytes_per_pixel = 3;
        let bytes_per_scanline = info.width * bytes_per_pixel;

        // convert_to_bytes
        let bdata: Vec<u8> = data
            .as_slice()
            .iter()
            .map(|val| Self::float_to_byte(*val))
            .collect();

        Self {
            bdata,
            bytes_per_pixel: bytes_per_pixel as usize,
            bytes_per_scanline: bytes_per_scanline as usize,
            width: info.width,
            height: info.height,
        }
    }

    // Return slice of the RGB bytes of pixel (x,y).
    // If the image data vector is empty returns magenta
    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        if self.bdata.is_empty() {
            return &MAGENTA;
        }

        let x = Self::clamp(x, 0, self.width as usize);
        let y = Self::clamp(y, 0, self.height as usize);

        let idx = y * self.bytes_per_scanline + x * self.bytes_per_pixel;

        &self.bdata[idx..idx + 3]
    }

    fn float_to_byte(val: f32) -> u8 {
        if val <= 0.0 {
            return 0;
        }
        if val >= 1.0 {
            return 255;
        }

        (256.0 * val) as u8
    }

    // Return value clamped to range [low, high)
    fn clamp(x: usize, low: usize, high: usize) -> usize {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }

        high - 1
    }
}
