use crate::{interval::Interval, vector::Vec3};

pub type Color = Vec3;

impl Color {
    pub const BLACK: Color = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const WHITE: Color = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn from_f32_slice(slice: &[f32]) -> Self {
        Self::new(slice[0].into(), slice[1].into(), slice[2].into())
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn color_to_string(color: Color, samples: i32) -> String {
    let scale = 1.0 / samples as f64;
    let intensity = Interval::new(0.0, 0.999);

    let r = linear_to_gamma(color.x * scale);
    let g = linear_to_gamma(color.y * scale);
    let b = linear_to_gamma(color.z * scale);

    let r = (intensity.clamp(r) * 255.999) as i32;
    let g = (intensity.clamp(g) * 255.999) as i32;
    let b = (intensity.clamp(b) * 255.999) as i32;

    format!("{r} {g} {b}\n")
}
