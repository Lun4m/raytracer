use crate::{
    material::Material,
    ray::Ray,
    vector::{dot, Vec3},
};

#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets hit record of normal vector,
    /// so that we can distinguish if we are inside or outside an object
    /// outward_normal assumed to have unit norm
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn update(&mut self, other: &HitRecord) {
        self.point = other.point.clone();
        self.normal = other.normal.clone();
        self.t = other.t;
        self.material = other.material.clone();
        self.front_face = other.front_face;
    }
}
