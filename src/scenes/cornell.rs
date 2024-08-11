use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    hittables::HittableList,
    material::{DiffuseLight, Lambertian},
    quad::{create_box, Quad, Shape},
    vector::Vec3,
    volumes::BvhNode,
};

pub fn cornell_box() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 800,
        samples: 200,
        max_depth: 50,
        background: Color::BLACK,
        vfov: 40.0,
        look_from: Vec3::new(278.0, 278.0, -800.0),
        look_at: Vec3::new(278.0, 278.0, 0.0),
        ..CameraConfig::default()
    });

    let red = Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::from_rgb(0.93, 0.93, 0.93));
    let green = Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::from_rgb(15.0, 15.0, 15.0));

    // (000) top right corner
    let world = HittableList::from_vec(vec![
        // light
        Arc::new(Quad::new(
            Vec3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            light,
            Shape::Square,
        )),
        // left
        Arc::new(Quad::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green,
            Shape::Square,
        )),
        // right
        Arc::new(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            red,
            Shape::Square,
        )),
        // bottom
        Arc::new(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone(),
            Shape::Square,
        )),
        // top
        Arc::new(Quad::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone(),
            Shape::Square,
        )),
        // behind
        Arc::new(Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone(),
            Shape::Square,
        )),
        // boxes
        create_box(
            Vec3::new(130.0, 0.0, 65.0),
            Vec3::new(295., 165., 230.),
            white.clone(),
        ),
        create_box(
            Vec3::new(265.0, 0.0, 295.0),
            Vec3::new(430., 330., 460.),
            white.clone(),
        ),
    ]);

    let world = BvhNode::from(world);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
