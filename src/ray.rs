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
        let sphere = Vec3::new(0.0, 0.0, -1.0);
        let t = self.hit_sphere(sphere, 0.5);
        if t > 0.0 {
            let normal = unit_vector(self.at(t) - sphere);
            return 0.5 * Color::from(normal + 1.0);
        }

        let unit_direction = unit_vector(self.direction);
        // LERP transformation
        let percent = 0.5 * (unit_direction.y + 1.0);
        (1.0 - percent) * Color::new(1.0, 1.0, 1.0) + percent * Color::new(0.5, 0.7, 1.0)
    }

    /// Solves a quadratic equation to determine if the sphere was hit by the ray
    fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.direction.len_squared();
        let half_b = dot(&oc, &self.direction);
        let c = oc.len_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        (half_b - discriminant.sqrt()) / a
    }
}
