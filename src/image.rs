use std::{env, fs, io};

use stb::image::Channels;

use crate::color::Color;

#[derive(Default)]
pub struct Image {
    bytes_per_pixel: usize,
    bytes_per_scanline: usize,
    // Linear floating point pixel data
    fdata: Vec<f32>,
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

        let bytes_per_pixel = info.components;
        let bytes_per_scanline = info.width * bytes_per_pixel;

        Self {
            fdata: data.into_vec(),
            bytes_per_pixel: bytes_per_pixel as usize,
            bytes_per_scanline: bytes_per_scanline as usize,
            width: info.width,
            height: info.height,
        }
    }

    // Return RGB color of pixel (x,y).
    // If the image data vector is empty returns magenta
    pub fn pixel_color(&self, x: usize, y: usize) -> Color {
        if self.fdata.is_empty() {
            return Color::new(1.0, 0.0, 1.0);
        }

        let x = Self::clamp(x, 0, self.width as usize);
        let y = Self::clamp(y, 0, self.height as usize);

        let idx = y * self.bytes_per_scanline + x * self.bytes_per_pixel;

        Color::from_f32_slice(&self.fdata[idx..idx + 3])
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
