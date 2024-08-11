use std::{f64::consts::PI, sync::Arc};

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{dot, Vec3, EPS},
    volumes::BoundingBox,
};

pub struct Sphere {
    center: Vec3,
    // Point the sphere center is moving towards if in motion
    direction: Option<Vec3>,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
    bbox: BoundingBox,
}

impl Sphere {
    pub fn new(
        center: Vec3,
        radius: f64,
        material: impl Material + Send + Sync + 'static,
    ) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = BoundingBox::from_extrema(center - rvec, center + rvec);
        Sphere {
            center,
            radius,
            direction: None,
            material: Arc::new(material),
            bbox,
        }
    }

    pub fn with_arc(
        center: Vec3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = BoundingBox::from_extrema(center - rvec, center + rvec);
        Sphere {
            center,
            radius,
            direction: None,
            material,
            bbox,
        }
    }

    pub fn new_in_motion(
        center1: Vec3,
        center2: Vec3,
        radius: f64,
        material: impl Material + Send + Sync + 'static,
    ) -> Self {
        let direction = Some(center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = BoundingBox::from_extrema(center1 - rvec, center1 + rvec);
        let bbox2 = BoundingBox::from_extrema(center2 - rvec, center2 + rvec);
        let bbox = BoundingBox::from_boxes(bbox1, bbox2);
        Self {
            center: center1,
            direction,
            radius,
            material: Arc::new(material),
            bbox,
        }
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        self.center + time * self.direction.unwrap_or_default()
    }

    // Get texture mapped coordinates of a given `point` on the sphere
    pub fn get_uv(point: Vec3) -> (f64, f64) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = 0.5 * phi / PI;
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let center = self.sphere_center(ray.time);

        let oc = center - ray.origin;
        let a = ray.direction.len_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();
        let mut root = (half_b - dsqrt) / a;

        // filter out negative values
        if !interval.surrounds(root) {
            root = (half_b + dsqrt) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - center) / self.radius;

        Some(HitRecord::new(
            ray,
            outward_normal,
            Self::get_uv(outward_normal),
            root,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}
