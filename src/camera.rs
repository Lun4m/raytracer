use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

use rand::random;
use rayon::prelude::*;

use crate::{
    color::{color_to_string, Color},
    hittables::Hittable,
    ray::Ray,
    vector::{cross, unit_vector, Vec3},
    world::World,
};

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: i32,
    /// Number of samples to compute for each pixel
    pub samples: i32,
    /// Maximum number of ray bounces
    pub max_depth: i32,
    /// Vertical viewing angle (field of view)
    pub vfov: f64,
    /// Point the camera is looking from
    pub look_from: Vec3,
    /// Point the camera is looking at
    pub look_at: Vec3,
    /// Camera relative "up" direction
    pub up_direction: Vec3,
    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from `look_from` to plane of perfect focus
    pub focus_dist: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 500,
            samples: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Vec3::default(),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up_direction: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel_00: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples: i32,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        // image setup
        let image_height = (config.image_width as f64 / config.aspect_ratio) as i32;
        let center = config.look_from;

        // viewport dimensions
        let theta = config.vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width = viewport_height * config.aspect_ratio;

        // Calculate camera frame basis vectors
        // (u ~= x, v ~= y, w ~= z)
        let w = unit_vector(&(&center - &config.look_at)); // opposite to view direction
        let u = unit_vector(&cross(&config.up_direction, &w));
        let v = cross(&w, &u);

        // vievport vectors
        let viewport_u = viewport_width * &u; // horizonatal left -> right
        let viewport_v = -viewport_height * &v; // vertical top -> down

        // Calculate horizontal and vertical pixel spacing vectors
        let pixel_delta_u = &viewport_u / config.image_width;
        let pixel_delta_v = &viewport_v / image_height;

        // Calculate location of upper left pixel
        let viewport_upperleft =
            &center - (config.focus_dist * w) - 0.5 * (viewport_u + viewport_v);
        let pixel_00 = viewport_upperleft + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        // Calculate defocus disk
        let defocus_radius = config.focus_dist * (0.5 * config.defocus_angle).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Camera {
            image_width: config.image_width,
            samples: config.samples,
            max_depth: config.max_depth,
            defocus_angle: config.defocus_angle,
            image_height,
            center,
            pixel_00,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
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
                        .map(|_| get_color(self.get_ray(i, j), &world, self.max_depth))
                        .sum();

                    color_to_string(pixel_color, self.samples)
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
        let ray_origin = &self.center + &self.defocus_disk_sample();

        let pixel_center = &self.pixel_00 + (i * &self.pixel_delta_u) + (j * &self.pixel_delta_v);
        let ray_target = pixel_center + self.pixel_sample_square();
        let ray_direction = ray_target - ray_origin.clone();
        let ray_time = random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        if self.defocus_angle <= 0.0 {
            return Vec3::default();
        }

        let vec = Vec3::random_in_unit_disk();
        (vec.x * &self.defocus_disk_u) + (vec.y * &self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random::<f64>();
        let py = -0.5 + random::<f64>();
        (px * &self.pixel_delta_u) + (py * &self.pixel_delta_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(CameraConfig::default())
    }
}

// TODO: get rid of recursion?
pub fn get_color(ray: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit_obj) = world.hit(&ray) {
        return match hit_obj.material.scatter(&ray, &hit_obj) {
            Some((ray_scattered, attenuation)) => {
                attenuation * get_color(ray_scattered, world, depth - 1)
            }
            None => Color::default(),
        };
    }

    // Sky box
    let unit_direction = unit_vector(&ray.direction);
    let percent = 0.5 * (unit_direction.y + 1.0);
    (1.0 - percent) * Color::white() + percent * Color::new(0.5, 0.7, 1.0)
}
