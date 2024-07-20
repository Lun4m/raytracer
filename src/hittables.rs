use std::sync::Arc;

use crate::{
    material::Material,
    ray::Ray,
    vector::{dot, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

/// Struct that keeps track of the hit point, the normal vector at that point,
/// the material of the hit object
pub struct HitRecord {
    pub distance: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets hit record of normal vector,
    /// so that we can distinguish if we are inside or outside an object
    /// outward_normal assumed to have unit norm
    pub fn new(ray: &Ray, normal: Vec3, distance: f64, material: Arc<dyn Material>) -> Self {
        let point = ray.at(distance);
        let front_face = dot(&ray.direction, &normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            distance,
            point,
            normal,
            material,
            front_face,
        }
    }
}
