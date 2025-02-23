use std::{collections::HashMap, env, process};

mod boundind_box;
mod camera;
mod color;
mod hittables;
mod image;
mod interval;
mod material;
mod perlin;
mod quad;
mod random;
mod ray;
mod scenes;
mod sphere;
mod texture;
mod utils;
mod vector;
mod volumes;

fn main() {
    let scene_names = HashMap::from([
        ("bouncing_spheres", scenes::bouncing_spheres as fn()),
        ("checkered_spheres", scenes::checkered_spheres as fn()),
        ("earth", scenes::earth as fn()),
        ("quads", scenes::quads as fn()),
        ("light", scenes::light as fn()),
        ("perlin", scenes::perlin_spheres as fn()),
        ("cornell_box", scenes::cornell_box as fn()),
        ("cornell_smoke", scenes::cornell_smoke as fn()),
        ("the_week_after", scenes::the_week_after as fn()),
    ]);

    let usage = || {
        println!("USAGE: raytracer-cpu <scene_name>\n\nValid scene names:");
        scene_names.keys().for_each(|s| println!("    - {s}"));
    };

    let scene = match env::args().nth(1) {
        Some(v) => v,
        None => {
            usage();
            process::exit(0)
        }
    };

    scene_names.get(scene.as_str()).map_or_else(usage, |f| f())
}
