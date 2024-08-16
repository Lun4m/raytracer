use std::sync::Arc;

use crate::{
    boundind_box::BvhNode,
    camera::{Camera, CameraConfig},
    color::Color,
    hittables::HittableList,
    material::{Dielectric, Lambertian, Metal},
    random,
    sphere::Sphere,
    texture::Checker,
    vector::Vec3,
};

pub fn bouncing_spheres() {
    // image_width, samples, and max_depth are the big performance hitter
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 800,
        samples: 50,
        max_depth: 10,
        vfov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        defocus_angle: 0.6,
        ..CameraConfig::default()
    });

    let checker = Checker::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new(Arc::new(checker));
    let mut world = HittableList::from_vec(vec![Arc::new(Sphere::new(
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

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                // Diffuse
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::from_albedo(albedo);
                    let new_center = center + Vec3::new(0.0, random::in_interval(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_in_motion(
                        center, new_center, 0.2, material,
                    )));
                    continue;
                }
                // Metal
                if choose_mat < 0.95 {
                    let albedo = Color::random_min_max(0.5, 1.0);
                    let fuzz = random::in_interval(0.0, 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                    continue;
                }
                // Dielectric
                let material = Dielectric::new(1.5);
                world.add(Arc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    // Bigger spheres
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::from_rgb(0.4, 0.2, 0.1),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::from_rgb(0.7, 0.6, 0.5, 0.0),
    )));

    let world = BvhNode::from(world);

    if let Err(e) = camera.render(world) {
        eprintln!("Failed while rendering with error: {e}")
    }
}
