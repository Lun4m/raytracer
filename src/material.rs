use rand::random;

use crate::{
    color::Color,
    hittables::HitRecord,
    ray::Ray,
    vector::{dot, unit_vector, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = &record.normal + Vec3::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal.clone();
        }

        Some((
            Ray::new(record.point.clone(), scatter_direction, ray.time),
            self.albedo.clone(),
        ))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = unit_vector(&reflect(&ray.direction, &record.normal))
            + self.fuzz * Vec3::random_in_unit_sphere();

        let ray_direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(record.point.clone(), ray_direction, ray.time);

        if dot(&scattered.direction, &record.normal) > 0.0 {
            return Some((scattered, self.albedo.clone()));
        }
        None
    }
}

pub struct Dielectric {
    eta: f64, // refraction_index
}

impl Dielectric {
    pub fn new(eta: f64) -> Self {
        Self { eta }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let eta_ratio = if record.front_face {
            1.0 / self.eta
        } else {
            self.eta
        };

        let unit_direction = unit_vector(&ray.direction);
        let out_direction = refract(&unit_direction, &record.normal, eta_ratio);

        let attenuation = Color::white();
        Some((
            Ray::new(record.point.clone(), out_direction, ray.time),
            attenuation,
        ))
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn reflectance(cos: f64, eta_ratio: f64) -> f64 {
    // Schlink's approximation
    let r = (1.0 - eta_ratio) / (1.0 + eta_ratio);
    let rsqrd = r * r;

    rsqrd + (1.0 - rsqrd) * (1.0 - cos).powi(5)
}

pub fn refract(v: &Vec3, n: &Vec3, eta_ratio: f64) -> Vec3 {
    let cos_theta = (-dot(v, n)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    // If the ray cannot be refracted it is reflected (total internal reflection)
    // Should only happen for materials that have eta < eta of the external medium
    let cannot_refract = sin_theta * eta_ratio > 1.0;
    // Takes care of materials that respond differtly with the angle
    let will_reflect = reflectance(cos_theta, eta_ratio) > random();
    if cannot_refract || will_reflect {
        return reflect(v, n);
    }

    let r_out_perp = eta_ratio * (v + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;

    // TODO: worth implementing ops_*_mut() methods?
    r_out_perp + r_out_parallel
}
