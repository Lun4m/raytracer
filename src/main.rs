use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

mod color;
mod vector;

use color::{write_color, Color};

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write_all(header.as_bytes()).unwrap();

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_width - 1) as f64,
                0.0,
            );
            write_color(&mut writer, pixel_color)
        }
    }

    Ok(())
}
