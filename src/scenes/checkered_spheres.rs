use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    material::Lambertian,
    sphere::Sphere,
    texture::Checker,
    vector::Vec3,
    world::World,
};

pub fn checkered_spheres() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 100,
        max_depth: 50,
        vfov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        ..CameraConfig::default()
    });

    let checker = Arc::new(Checker::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let world = World::from_vec(vec![
        Arc::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        )),
        Arc::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        )),
    ]);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
