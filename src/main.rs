mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod random;
mod ray;
mod sphere;
mod vector;
mod volumes;
mod world;

use std::sync::Arc;

use camera::{Camera, CameraConfig};
use color::Color;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vector::Vec3;
use volumes::BvhNode;
use world::World;

fn main() {
    // image_width, samples, and max_depth are the big performance hitter
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 50,
        max_depth: 10,
        vfov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut world = World::from_vec(vec![Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::float();
            let center = Vec3::new(
                a as f64 + 0.9 * random::float(),
                0.2,
                b as f64 + 0.9 * random::float(),
            );

            if (&center - &Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                // Diffuse
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    let new_center = &center + Vec3::new(0.0, random::in_interval(0.0, 0.5), 0.0);
                    world.add(Sphere::new_in_motion(center, new_center, 0.2, material));
                    continue;
                }
                // Metal
                if choose_mat < 0.95 {
                    let albedo = Color::random_min_max(0.5, 1.0);
                    let fuzz = random::in_interval(0.0, 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, material));
                    continue;
                }
                // Dielectric
                let material = Dielectric::new(1.5);
                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    // Bigger spheres
    world.add(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    let world = BvhNode::from_world(world).into();

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
