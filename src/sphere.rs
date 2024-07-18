use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{dot, Vec3},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<(f64, Vec3, Material)> {
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

        // TODO: do this outside?
        // Exclude objects that are outside the interval
        if !hit_range.surrounds(root) {
            root = (half_b + dsqrt) / a;
            if !hit_range.surrounds(root) {
                return None;
            }
        }

        let outward_normal = (&ray.at(root) - &self.center) / self.radius;
        Some((root, outward_normal, self.material.clone()))
    }
}
