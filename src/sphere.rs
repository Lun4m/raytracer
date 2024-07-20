use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vector::{dot, Vec3, EPS},
};

pub struct Sphere {
    center: Vec3,
    // Point the sphere center is moving towards if in motion
    direction: Option<Vec3>,
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
            direction: None,
            material: Arc::new(material),
        }
    }

    pub fn new_in_motion(
        center1: Vec3,
        center2: Vec3,
        radius: f64,
        material: impl Material + Send + Sync + 'static,
    ) -> Self {
        let direction = Some(&center2 - &center1);
        Self {
            center: center1,
            direction,
            radius,
            material: Arc::new(material),
        }
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        &self.center + time * self.direction.as_ref().unwrap_or(&Vec3::default())
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let center = self.sphere_center(ray.time);

        let oc = &center - &ray.origin;
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

        let outward_normal = (&ray.at(root) - center) / self.radius;

        Some(HitRecord::new(
            ray,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
