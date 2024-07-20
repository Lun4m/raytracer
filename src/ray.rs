use crate::vector::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }
    pub fn at(&self, distance: f64) -> Vec3 {
        &self.origin + distance * &self.direction
    }
}
