use std::{env::args, process::exit};

mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod random;
mod ray;
mod scenes;
mod sphere;
mod texture;
mod vector;
mod volumes;
mod world;

const SCENE_NAMES: [&str; 2] = ["bouncing_spheres", "checkered_spheres"];

fn usage() {
    println!("USAGE: raytracer <scene_name>\n\nValid scene names:");
    SCENE_NAMES.iter().for_each(|s| println!("    - {s}"));
}

fn main() {
    let scene = match args().nth(1) {
        Some(v) => v,
        None => {
            usage();
            exit(0)
        }
    };

    match scene.as_str() {
        "bouncing_spheres" => scenes::bouncing_spheres(),
        "checkered_spheres" => scenes::checkered_spheres(),
        _ => usage(),
    }
}
