use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};

mod color;
mod hittables;
mod ray;
mod sphere;
mod vector;

use color::write_color;
use hittables::HitList;
use ray::Ray;
use sphere::Sphere;
use vector::Vec3;

fn main() -> std::io::Result<()> {
    // image setup
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // camera setup
    let focal_len = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vec3::default();

    // vievport vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upperleft =
        camera_center - Vec3::new(0.0, 0.0, focal_len) - 0.5 * (viewport_u + viewport_v);

    let pixel_00 = viewport_upperleft + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);

    let header = format!("P3\n{image_width} {image_height}\n255\n");
    writer.write_all(header.as_bytes()).unwrap();

    let mut world = HitList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel_00 + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            // TODO: is it worth it to pass references here insted of deriving clone on Vec3?
            let ray = Ray::new(camera_center, ray_direction);
            write_color(&mut writer, ray.color(&world))
        }
    }

    print!("\rDone.                   \n");
    Ok(())
}
