use crate::{
    color::Color,
    hittables::{HitRecord, Hittable},
    interval::Interval,
    vector::{unit_vector, Vec3},
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

    pub fn color(&self, world: &impl Hittable, depth: i32) -> Color {
        let mut record = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }

        if world.hit(self, Interval::positive(), &mut record) {
            let direction = record.normal + Vec3::random_unit_vector();
            return 0.5 * Ray::new(record.point, direction).color(world, depth - 1);
        }

        let unit_direction = unit_vector(self.direction);
        // LERP transformation
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::full() + percent * Color::new(0.5, 0.7, 1.0)
    }
}
