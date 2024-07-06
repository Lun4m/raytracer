use std::{fs::File, io::BufWriter, io::Write};

use crate::{interval::Interval, vector::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn full() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}

pub fn write_color(writer: &mut BufWriter<File>, color: Color, samples: i32) {
    let scale = 1.0 / samples as f64;
    let intensity = Interval::new(0.0, 0.999);

    let r = intensity.clamp(color.x * scale);
    let g = intensity.clamp(color.y * scale);
    let b = intensity.clamp(color.z * scale);

    let r = (r * 255.999) as i32;
    let g = (g * 255.999) as i32;
    let b = (b * 255.999) as i32;

    let line = format!("{r} {g} {b}\n");
    writer.write_all(line.as_bytes()).unwrap();
}
