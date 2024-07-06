use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

use crate::{color::write_color, hittables::HitList, ray::Ray, vector::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    image_height: i32,
    center: Vec3,
    pixel_00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        // image setup
        let image_height = (image_width as f64 / aspect_ratio) as i32;

        // camera setup
        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let center = Vec3::default();

        // vievport vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upperleft =
            center - Vec3::new(0.0, 0.0, focal_len) - 0.5 * (viewport_u + viewport_v);

        let pixel_00 = viewport_upperleft + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: HitList) -> std::io::Result<()> {
        let file = File::create("out.ppm")?;
        let mut writer = BufWriter::new(file);

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        writer.write_all(header.as_bytes()).unwrap();
        for j in 0..self.image_height {
            print!("\rScanlines remaining: {}", self.image_height - j);
            stdout().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel_00 + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;

                // TODO: is it worth it to pass references here insted of deriving clone on Vec3?
                let ray = Ray::new(self.center, ray_direction);
                write_color(&mut writer, ray.color(&world))
            }
        }

        print!("\rDone.                   \n");
        Ok(())
    }
}
