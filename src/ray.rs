use crate::{
    color::Color,
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

    pub fn color(&self) -> Color {
        if self.hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(self.direction);
        // LERP transformation
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::new(1.0, 1.0, 1.0) + percent * Color::new(0.5, 0.7, 1.0)
    }

    /// Solves a quadratic equation to determine if the sphere was hit by the ray
    fn hit_sphere(&self, center: Vec3, radius: f64) -> bool {
        let oc = center - self.origin;
        let a = dot(&self.direction, &self.direction);
        let b = -2.0 * dot(&oc, &self.direction);
        let c = dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
