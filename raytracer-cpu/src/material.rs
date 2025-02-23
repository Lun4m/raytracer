use std::sync::Arc;

use crate::{
    color::Color,
    hittables::HitRecord,
    random,
    ray::Ray,
    texture::{ArcTexture, SolidColor},
    vector::{dot, unit_vector, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;

    #[allow(unused_variables)]
    fn emit(&self, uv: (f64, f64), point: Vec3) -> Color {
        Color::BLACK
    }
}

pub type ArcMaterial = Arc<dyn Material + Send + Sync>;

pub struct Lambertian {
    texture: ArcTexture,
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            texture: Arc::new(SolidColor::default()),
        }
    }
}

impl Lambertian {
    pub fn from_albedo(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            texture: Arc::new(SolidColor::from_rgb(r, g, b)),
        }
    }

    pub fn new(texture: ArcTexture) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = record.normal + Vec3::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some((
            // scattered ray
            Ray::new(record.point, scatter_direction, ray.time),
            // attenuation
            self.texture.value(record.uv, record.point),
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

    pub fn from_rgb(rgb: (f64, f64, f64), fuzz: f64) -> Self {
        Self {
            albedo: Color::new(rgb.0, rgb.1, rgb.2),
            fuzz,
        }
    }

    fn reflectance(&self, cos: f64) -> Color {
        // Schlink's approximation for metals
        self.albedo + (Color::WHITE - self.albedo) * (1.0 - cos).powi(5)
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = unit_vector(reflect(ray.direction, record.normal))
            + self.fuzz * Vec3::random_in_unit_sphere();

        let ray_direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(record.point, ray_direction, ray.time);

        let cosine = dot(scattered.direction, record.normal);
        if cosine > 0.0 {
            return Some((scattered, self.reflectance(cosine)));
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

        let unit_direction = unit_vector(ray.direction);
        let out_direction = refract(unit_direction, record.normal, eta_ratio);

        let attenuation = Color::WHITE;
        Some((Ray::new(record.point, out_direction, ray.time), attenuation))
    }
}

pub struct DiffuseLight {
    texture: ArcTexture,
}

impl DiffuseLight {
    pub fn _new(texture: ArcTexture) -> Self {
        Self { texture }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            texture: Arc::new(SolidColor::from_rgb(r, g, b)),
        }
    }

    pub fn _from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emit(&self, uv: (f64, f64), point: Vec3) -> Color {
        self.texture.value(uv, point)
    }
}

pub struct Isotropic {
    texture: ArcTexture,
}

impl Isotropic {
    pub fn new(texture: ArcTexture) -> Self {
        Self { texture }
    }
    pub fn _from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            texture: Arc::new(SolidColor::from_rgb(r, g, b)),
        }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        Some((
            Ray::new(record.point, Vec3::random_in_unit_sphere(), ray.time),
            self.texture.value(record.uv, record.point),
        ))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

fn reflectance(cos: f64, eta_ratio: f64) -> f64 {
    // Schlink's approximation
    let r = (1.0 - eta_ratio) / (1.0 + eta_ratio);
    let rsqrd = r * r;

    rsqrd + (1.0 - rsqrd) * (1.0 - cos).powi(5)
}

fn refract(v: Vec3, n: Vec3, eta_ratio: f64) -> Vec3 {
    let cos_theta = (-dot(v, n)).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    // If the ray cannot be refracted it is reflected (total internal reflection)
    // Should only happen for materials that have eta < eta of the external medium
    let cannot_refract = sin_theta * eta_ratio > 1.0;
    // Takes care of materials that respond differtly with the angle
    let will_reflect = reflectance(cos_theta, eta_ratio) > random::float();
    if cannot_refract || will_reflect {
        return reflect(v, n);
    }

    let r_out_perp = eta_ratio * (v + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * n;

    r_out_perp + r_out_parallel
}
