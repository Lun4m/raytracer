use crate::{
    hittables::{HitRecord, Hittable},
    ray::Ray,
    vector::{dot, Vec3},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.len_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let dsqrt = discriminant.sqrt();
        let mut root = (half_b - dsqrt) / a;
        if root <= tmin || root > tmax {
            root = (half_b + dsqrt) / a;
            if root <= tmin || root > tmax {
                return false;
            }
        }
        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        true
    }
}
