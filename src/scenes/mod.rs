mod bouncing_spheres;
mod checkered_spheres;

pub use bouncing_spheres::bouncing_spheres;
pub use checkered_spheres::checkered_spheres;

enum Scenes {
    BouncingSpheres,
    CheckeredSpheres,
}
