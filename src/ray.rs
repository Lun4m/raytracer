use std::f64::INFINITY;

use crate::{
    color::Color,
    hittables::{HitRecord, Hittable},
    vector::{dot, unit_vector, Vec3},
};

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &impl Hittable) -> Color {
        let mut record = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut record) {
            return 0.5 * (record.normal + Color::full());
        }

        let unit_direction = unit_vector(self.direction);
        // LERP transformation
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::full() + percent * Color::new(0.5, 0.7, 1.0)
    }
}
