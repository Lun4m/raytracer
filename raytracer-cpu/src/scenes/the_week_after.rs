use std::sync::Arc;

use crate::{
    boundind_box::BvhNode,
    camera::{Camera, CameraConfig},
    color::Color,
    hittables::{HittableList, RotateY, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    quad::{create_box, Quad, Shape},
    random,
    sphere::Sphere,
    texture::{ImageTexture, NoiseTexture},
    vector::Vec3,
    volumes::ConstantMedium,
};

pub fn final_scene() {
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 800,
        samples: 1000,
        max_depth: 50,
        vfov: 40.0,
        look_from: Vec3::new(478.0, 278.0, -600.0),
        look_at: Vec3::new(278.0, 278.0, 0.0),
        background: Color::BLACK,
        ..CameraConfig::default()
    });

    let ground = Arc::new(Lambertian::from_rgb(0.48, 0.83, 0.53));
    let boxes_per_side = 20;

    let mut boxes_1 = HittableList::new();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let width = 100.0;

            let x0 = -1000.0 + width * i as f64;
            let y0 = 0.0;
            let z0 = -1000.0 + width * j as f64;

            let x1 = x0 + width;
            let y1 = random::in_interval(1.0, 101.0);
            let z1 = z0 + width;

            boxes_1.add(create_box(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            ))
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::from(boxes_1)));

    let light = Arc::new(DiffuseLight::from_rgb(7., 7., 7.));
    world.add(Arc::new(Quad::new(
        Vec3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        light,
        Shape::Square,
    )));

    let center_1 = Vec3::new(400., 400., 200.);
    let center_2 = center_1 + Vec3::new(30., 0., 0.);
    let sphere_material = Arc::new(Lambertian::from_rgb(0.7, 0.3, 0.1));
    world.add(Arc::new(Sphere::new_in_motion(
        center_1,
        center_2,
        50.,
        sphere_material.clone(),
    )));

    let dielectric = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        dielectric.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::from_rgb((0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(Vec3::new(360., 150., 145.), 70., dielectric));
    world.add(boundary.clone());

    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::default(),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary,
        0.0001,
        Color::WHITE,
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400., 200., 400.),
        100.,
        emat,
    )));

    let perlin_texture = NoiseTexture::new(0.2);
    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(Arc::new(perlin_texture))),
    )));

    let mut boxes_2 = HittableList::new();
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        boxes_2.add(Arc::new(Sphere::new(
            Vec3::random_min_max(0., 165.),
            10.,
            white.clone(),
        )))
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::from(boxes_2)), 15.)),
        Vec3::new(-100., 270., 395.),
    )));

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
