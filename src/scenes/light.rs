use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    material::{DiffuseLight, Lambertian},
    quad::{Quad, Shape},
    sphere::Sphere,
    vector::Vec3,
    volumes::BvhNode,
    world::World,
};

pub fn light() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 100,
        max_depth: 50,
        background: Color::BLACK,
        vfov: 20.0,
        look_from: Vec3::new(26.0, 3.0, 6.0),
        look_at: Vec3::new(0.0, 2.0, 0.0),
        ..CameraConfig::default()
    });

    let red = Arc::new(Lambertian::from_rgb(1.0, 0.2, 0.2));
    // TODO: 4, 4, 4 ???
    let diff_light = Arc::new(DiffuseLight::from_rgb(4.0, 4.0, 4.0));

    let world = World::from_vec(vec![
        Arc::new(Sphere::with_arc(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            red.clone(),
        )),
        Arc::new(Sphere::with_arc(Vec3::new(0.0, 2.0, 0.0), 2.0, red)),
        Arc::new(Quad::new(
            Vec3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            diff_light,
            Shape::Square,
        )),
    ]);

    let world = BvhNode::from_world(world);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
