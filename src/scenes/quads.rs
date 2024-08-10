use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    material::Lambertian,
    quad::Quad,
    vector::Vec3,
    volumes::BvhNode,
    world::World,
};

pub fn quads() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 800,
        samples: 100,
        max_depth: 50,
        vfov: 80.0,
        look_from: Vec3::new(0.0, 0.0, 9.0),
        ..CameraConfig::default()
    });

    let left_red = Lambertian::from_rgb(1.0, 0.2, 0.2);
    let back_green = Lambertian::from_rgb(0.2, 1.0, 0.2);
    let right_blue = Lambertian::from_rgb(0.2, 0.2, 1.0);
    let upper_orange = Lambertian::from_rgb(1.0, 0.5, 0.0);
    let lower_teal = Lambertian::from_rgb(0.2, 0.8, 0.8);

    let world = World::from_vec(vec![
        Arc::new(Quad::new(
            Vec3::new(-3.0, -2.0, 5.0),
            Vec3::new(0.0, 0.0, -4.0),
            Vec3::new(0.0, 4.0, 0.0),
            left_red,
        )),
        Arc::new(Quad::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            back_green,
        )),
        Arc::new(Quad::new(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            right_blue,
        )),
        Arc::new(Quad::new(
            Vec3::new(-2.0, 3.0, 1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            upper_orange,
        )),
        Arc::new(Quad::new(
            Vec3::new(-2.0, -3.0, 5.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -4.0),
            lower_teal,
        )),
    ]);

    let world = BvhNode::from_world(world);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
