mod camera;
mod color;
mod hittables;
mod interval;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use hittables::HitList;
use sphere::Sphere;
use vector::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples = 50;
    let max_depth = 10;
    let camera = Camera::new(aspect_ratio, image_width, samples, max_depth);

    let mut world = HitList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    if let Err(e) = camera.render(world) {
        eprintln!("Render failed with error: {}", e)
    }
}
