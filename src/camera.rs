use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

use rand::random;
use rayon::prelude::*;

use crate::{
    color::{get_color, Color},
    ray::Ray,
    vector::{cross, unit_vector, Vec3},
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
    // Vertical viewing angle (field of view)
    // vfov: f64,
    // Point the camera is looking from
    // look_from: Vec3,
    // Point the camera is looking at
    // look_at: Vec3,
    // Camera relative "up" direction
    // up_direction: Vec3,
    // Camera frame basis vectors
    // basis: [Vec4; 3],
    //
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Vec3,
        look_at: Vec3,
        up_direction: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        // image setup
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let center = look_from;

        // viewport dimensions
        // let focal_len = (look_from - look_at).len();
        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate camera frame basis vectors
        // (u ~= x, v ~= y, w ~= z)
        let w = unit_vector(look_from - look_at); // opposite to view direction
        let u = unit_vector(cross(&up_direction, &w));
        let v = cross(&w, &u);

        // vievport vectors
        let viewport_u = viewport_width * u; // horizonatal left -> right
        let viewport_v = -viewport_height * v; // vertical top -> down

        // Calculate horizontal and vertical pixel spacing vectors
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate location of upper left pixel
        let viewport_upperleft = center - (focus_dist * w) - 0.5 * (viewport_u + viewport_v);
        let pixel_00 = viewport_upperleft + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate defocus disk
        let defocus_radius = focus_dist * (0.5 * defocus_angle).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

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
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    pub fn render(&self, world: World) -> std::io::Result<()> {
        let file = File::create("out.ppm")?;
        let mut writer = BufWriter::new(file);

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        writer.write_all(header.as_bytes())?;

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {:>3}", self.image_height - j);
            stdout().flush()?;

            (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let pixel_color = (0..self.samples)
                        // .into_par_iter()
                        .map(|_| self.get_ray(i, j).color(&world, self.max_depth))
                        .sum();

                    get_color(pixel_color, self.samples)
                })
                .collect::<Vec<String>>()
                .into_iter()
                // TODO: fix this unwrap?
                .for_each(|color| writer.write_all(color.as_bytes()).unwrap());
        }

        println!("\rDone!                   ");
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let ray_origin = self.center + self.defocus_disk_sample();

        let pixel_center = self.pixel_00 + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let ray_target = pixel_center + self.pixel_sample_square();

        // TODO: is it worth it to pass references here insted of deriving clone on Vec3?
        Ray::new(ray_origin, ray_target - ray_origin)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        if self.defocus_angle <= 0.0 {
            return Vec3::default();
        }

        let vec = Vec3::random_in_unit_disk();
        (vec.x * self.defocus_disk_u) + (vec.y * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
