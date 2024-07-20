use std::sync::Arc;

use crate::{
    material::{Hittable, Lambertian},
    ray::Ray,
    vector::{dot, Vec3},
};

/// Struct that keeps track of the hit point, the normal vector at point,
/// the material of the hit object
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Hittable>,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            material: Arc::new(Lambertian::default()),
            front_face: false,
        }
    }
}

impl HitRecord {
    /// Sets hit record of normal vector,
    /// so that we can distinguish if we are inside or outside an object
    /// outward_normal assumed to have unit norm
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
