use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    hittables::HittableList,
    material::Lambertian,
    sphere::Sphere,
    texture::ImageTexture,
    vector::Vec3,
};

pub fn earth() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 100,
        max_depth: 50,
        vfov: 20.0,
        look_from: Vec3::new(0.0, 0.0, 12.0),
        ..CameraConfig::default()
    });

    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Lambertian::new(Arc::new(earth_texture));
    let globe = HittableList::from_vec(vec![Arc::new(Sphere::new(
        Vec3::default(),
        2.0,
        earth_surface,
    ))]);

    if let Err(e) = camera.render(globe) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
