use crate::{
    color::Color,
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
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
    pub fn color(&self) -> Color {
        let unit_direction = unit_vector(self.direction);
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::new(1.0, 1.0, 1.0) + percent * Color::new(0.5, 0.7, 1.0)
    }
}
