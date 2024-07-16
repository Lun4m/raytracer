mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector;
mod world;

use camera::Camera;
use color::Color;
use material::Material;
use sphere::Sphere;
use vector::Vec3;
use world::World;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples = 50;
    let max_depth = 10;
    let camera = Camera::new(aspect_ratio, image_width, samples, max_depth);

    let ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let left = Material::Dielectric {
        refraction_index: 1.5,
    };
    let right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    let mut world = World::new();
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right));

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
