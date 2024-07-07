use crate::{
    color::Color,
    hittables::HitRecord,
    ray::Ray,
    vector::{dot, reflect, unit_vector, Vec3},
};

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal((Color, f64)),
    Empty,
}

impl Default for Material {
    fn default() -> Self {
        Self::Empty
    }
}

impl Material {
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(albedo) => {
                let scatter_direction = record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    return Some((Ray::new(record.point, record.normal), *albedo));
                }

                Some((Ray::new(record.point, scatter_direction), *albedo))
            }
            Material::Metal((albedo, fuzz)) => {
                let reflected = unit_vector(reflect(&ray.direction, &record.normal))
                    + *fuzz * Vec3::random_unit_vector();

                let scattered =
                    Ray::new(record.point, reflected + *fuzz * Vec3::random_unit_vector());

                if dot(&scattered.direction, &record.normal) > 0.0 {
                    return Some((scattered, *albedo));
                }
                None
            }
            Material::Empty => unreachable!(),
        }
    }
}
