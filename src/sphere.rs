use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vector::{dot, Vec3, EPS},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f64,
        material: impl Material + Send + Sync + 'static,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material: Arc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // (f64, Vec3, Arc<dyn Hittable + Send + Sync>)> {
        let oc = &self.center - &ray.origin;
        let a = ray.direction.len_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();
        let mut root = (half_b - dsqrt) / a;

        // filter out negative values
        if root <= EPS {
            root = (half_b + dsqrt) / a;
            if root <= EPS {
                return None;
            }
        }

        let outward_normal = (&ray.at(root) - &self.center) / self.radius;

        Some(HitRecord::new(
            ray,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
