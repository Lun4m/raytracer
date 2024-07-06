use std::{fs::File, io::BufWriter, io::Write};

use crate::vector::Vec3;

pub type Color = Vec3;

pub fn write_color(writer: &mut BufWriter<File>, color: Color) {
    let r = (color.x * 255.999) as i32;
    let g = (color.y * 255.999) as i32;
    let b = (color.z * 255.999) as i32;

    let line = format!("{r} {g} {b}\n");
    writer.write_all(line.as_bytes()).unwrap();
}
