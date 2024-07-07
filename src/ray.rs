use crate::{
    color::Color,
    hittables::HitRecord,
    interval::Interval,
    vector::{unit_vector, Vec3},
    world::World,
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

    // TODO: refactor this function, it's a bit messy with the default HitRecord
    pub fn color(&self, world: &World, depth: i32) -> Color {
        let mut record = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }

        if world.hit(self, Interval::positive(), &mut record) {
            match record.material.scatter(self, &record) {
                Some((ray_scattered, attenuation)) => {
                    return attenuation * ray_scattered.color(world, depth - 1)
                }
                None => return Color::default(),
            }
        }

        let unit_direction = unit_vector(self.direction);
        // LERP transformation
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::full() + percent * Color::new(0.5, 0.7, 1.0)
    }
}
