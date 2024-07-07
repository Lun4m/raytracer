use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

use rand::random;
use rayon::prelude::*;

use crate::{
    color::{get_color, write_color, Color},
    ray::Ray,
    vector::Vec3,
    world::World,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,

    image_height: i32,
    center: Vec3,
    pixel_00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Number of ray casted per each pixel
    samples: i32,
    // Max number of ray bounces
    max_depth: i32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples: i32, max_depth: i32) -> Self {
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
            samples,
            max_depth,
        }
    }

    pub fn render(&self, world: World) -> std::io::Result<()> {
        let file = File::create("out.ppm")?;
        let mut writer = BufWriter::new(file);

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        writer.write_all(header.as_bytes()).unwrap();

        let lines: Vec<&[u8]> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                // for j in 0..self.image_height {
                // TODO: this print won't work in parallel
                print!("\rScanlines remaining: {}", self.image_height - j);
                stdout().flush().unwrap();

                let out = Vec::new();
                for i in 0..self.image_width {
                    let mut pixel_color = Color::default();

                    for _ in 0..self.samples {
                        let ray = self.get_ray(i, j);
                        pixel_color += ray.color(&world, self.max_depth);
                    }
                    // TODO: to use par_iter we need to collect the colors
                    // and then write them at the end outside this closure

                    // write_color(&mut writer, pixel_color, self.samples)
                    out.push(get_color(pixel_color, self.samples).as_bytes())
                }
                out
            })
            .collect()
            .into_iter()
            .flatten()
            .collect();

        print!("\rDone.                   \n");
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel_00 + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let ray_direction = pixel_center + self.pixel_sample_square();

        // TODO: is it worth it to pass references here insted of deriving clone on Vec3?
        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
