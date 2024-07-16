use crate::{
    color::Color,
    hittables::HitRecord,
    ray::Ray,
    vector::{dot, reflect, refract, unit_vector, Vec3},
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
                let scatter_direction = record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    return Some((Ray::new(record.point, record.normal), *albedo));
                }

                Some((Ray::new(record.point, scatter_direction), *albedo))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = unit_vector(reflect(&ray.direction, &record.normal))
                    + *fuzz * Vec3::random_unit_vector();

                let scattered =
                    Ray::new(record.point, reflected + *fuzz * Vec3::random_unit_vector());

                if dot(&scattered.direction, &record.normal) > 0.0 {
                    return Some((scattered, *albedo));
                }
                None
            }
            Material::Dielectric {
                mut refraction_index,
            } => {
                let attenuation = Color::full();
                if record.front_face {
                    refraction_index = 1.0 / refraction_index
                }
                let refracted = refract(
                    &unit_vector(ray.direction),
                    &record.normal,
                    refraction_index,
                );

                Some((Ray::new(record.point, refracted), attenuation))
            }
        }
    }
}
