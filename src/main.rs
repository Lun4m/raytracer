mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector;
mod world;

use camera::{Camera, CameraConfig};
use color::Color;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vector::Vec3;
use world::World;

fn main() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 50,
        max_depth: 10,
        vfov: 20.0,
        look_from: Vec3::new(-2.0, 2.0, 4.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        focus_dist: 3.4,
    });

    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left = Dielectric::new(
        // 1.0 / 1.333, // air / water => bubble sphere
        1.5, // glass
    );
    let right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let small_right = Metal::new(Color::new(0.2, 0.9, 0.2), 3.0);
    let small_left = Metal::new(Color::new(0.2, 0.2, 0.9), 3.0);
    let small_center = Metal::new(Color::new(0.9, 0.2, 0.2), 3.0);

    let world = World::from(vec![
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground),
        Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5, center),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left),
        // TODO: everything goes black if a metal sphere is inside another??
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right),
        //
        Sphere::new(Vec3::new(0.3, -0.3, -1.0), 0.2, small_right),
        Sphere::new(Vec3::new(0.0, -0.4, -0.7), 0.1, small_center),
        Sphere::new(Vec3::new(-0.3, -0.35, -1.0), 0.15, small_left),
    ]);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
