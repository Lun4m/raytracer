use std::sync::Arc;

use rand_distr::num_traits::Float;

use crate::{color::Color, image::Image, interval::Interval, perlin::Perlin, vector::Vec3};

pub type ArcTexture = Arc<dyn Texture + Send + Sync>;

pub trait Texture {
    // uv are the texture coordinates
    fn value(&self, uv: (f64, f64), point: Vec3) -> Color;
}

// annoying orphan rule
// impl Default for Arc<dyn Texture> {
//     fn default() -> Self {
//         SolidColor::default()
//     }
// }

#[derive(Default)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: (f64, f64), _: Vec3) -> Color {
        self.albedo
    }
}

pub struct Checker {
    inv_scale: f64,
    even: ArcTexture,
    odd: ArcTexture,
}

impl Checker {
    // fn new(scale: f64, even: ArcTexture, odd: ArcTexture) -> Self {
    //     Self {
    //         inv_scale: 1.0 / scale,
    //         even,
    //         odd,
    //     }
    // }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, uv: (f64, f64), point: Vec3) -> Color {
        let x = (self.inv_scale * point.x).floor() as i32;
        let y = (self.inv_scale * point.y).floor() as i32;
        let z = (self.inv_scale * point.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            return self.even.value(uv, point);
        }

        self.odd.value(uv, point)
    }
}

pub struct ImageTexture {
    img: Image,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            img: Image::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: (f64, f64), _: Vec3) -> Color {
        if self.img.height <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(uv.0);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(uv.1);

        let i = (u * self.img.width as f64) as usize;
        let j = (v * self.img.height as f64) as usize;

        self.img.pixel_color(i, j)
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    // resolution parameter
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(256),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: (f64, f64), point: Vec3) -> Color {
        0.5 * Color::white()
            * (1.0 + (self.scale * (point.z + self.noise.turbulence(point, 7))).sin())
    }
}
