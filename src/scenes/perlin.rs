use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    hittables::HittableList,
    material::Lambertian,
    sphere::Sphere,
    texture::NoiseTexture,
    vector::Vec3,
};

pub fn perlin_spheres() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 100,
        max_depth: 50,
        vfov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        ..CameraConfig::default()
    });

    let perlin_texture = NoiseTexture::new(4.0);
    let material = Arc::new(Lambertian::new(Arc::new(perlin_texture)));

    let globe = HittableList::from_vec(vec![
        Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            material.clone(),
        )),
        Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, material)),
    ]);

    if let Err(e) = camera.render(globe) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
