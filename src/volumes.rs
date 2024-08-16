use std::sync::Arc;

use rand_distr::num_traits::Float;

use crate::{
    boundind_box::BoundingBox,
    color::Color,
    hittables::{ArcHittable, HitRecord, Hittable},
    interval::Interval,
    material::{ArcMaterial, Isotropic},
    random,
    ray::Ray,
    texture::ArcTexture,
    vector::Vec3,
};

pub struct ConstantMedium {
    boundary: ArcHittable,
    neg_inv_density: f64,
    phase_function: ArcMaterial,
}

impl ConstantMedium {
    pub fn new(boundary: ArcHittable, density: f64, texture: ArcTexture) -> Self {
        let neg_inv_density = -1.0 / density;
        let phase_function = Arc::new(Isotropic::new(texture));
        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }

    pub fn from_color(boundary: ArcHittable, density: f64, albedo: Color) -> Self {
        let neg_inv_density = -1.0 / density;
        let phase_function = Arc::new(Isotropic::from_color(albedo));
        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut record_1 = self.boundary.hit(ray, Interval::_universe())?;

        let mut record_2 = self
            .boundary
            .hit(ray, Interval::with_min(record_1.distance + 0.0001))?;

        record_1.distance = record_1.distance.max(interval.min);
        record_2.distance = record_2.distance.min(interval.max);

        if record_1.distance >= record_2.distance {
            return None;
        }

        if record_1.distance < 0.0 {
            record_1.distance = 0.0
        }

        let ray_len = ray.direction.len();
        let distance_inside_boundary = (record_2.distance - record_1.distance) * ray_len;
        let hit_distance = self.neg_inv_density * random::float().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        Some(HitRecord::new(
            ray,
            Vec3::X, // arbitrary
            (0.0, 0.0),
            record_1.distance + hit_distance / ray_len,
            self.phase_function.clone(),
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.boundary.bounding_box()
    }
}
