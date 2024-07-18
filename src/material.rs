use rand::random;

use crate::{
    color::Color,
    hittables::HitRecord,
    ray::Ray,
    vector::{dot, unit_vector, Vec3},
};

#[derive(Clone)]
pub enum Material {
    Vacuum,
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Self::Vacuum
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Vacuum => unreachable!(),
            Material::Lambertian { albedo } => {
                let scatter_direction = &record.normal + Vec3::random_in_unit_sphere();

                if scatter_direction.near_zero() {
                    return Some((
                        Ray::new(record.point.clone(), record.normal.clone()),
                        albedo.clone(),
                    ));
                }

                Some((
                    Ray::new(record.point.clone(), scatter_direction),
                    albedo.clone(),
                ))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = unit_vector(&reflect(&ray.direction, &record.normal))
                    + *fuzz * Vec3::random_in_unit_sphere();

                let ray_direction = reflected + *fuzz * Vec3::random_in_unit_sphere();
                let scattered = Ray::new(record.point.clone(), ray_direction);

                if dot(&scattered.direction, &record.normal) > 0.0 {
                    return Some((scattered, albedo.clone()));
                }
                None
            }
            Material::Dielectric {
                refraction_index: eta,
            } => {
                let eta_ratio = if record.front_face { 1.0 / eta } else { *eta };

                let unit_direction = unit_vector(&ray.direction);
                let out_direction = refract(&unit_direction, &record.normal, eta_ratio);

                let attenuation = Color::white();
                Some((Ray::new(record.point.clone(), out_direction), attenuation))
            }
        }
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
