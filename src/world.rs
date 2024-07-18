use crate::{hittables::HitRecord, interval::Interval, ray::Ray, sphere::Sphere};

pub struct World {
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn _new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn from(objects: Vec<Sphere>) -> Self {
        Self { objects }
    }

    pub fn _add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }

    // TODO: feels like this could be implemented better
    pub fn hit(&self, ray: &Ray, hit_range: Interval) -> Option<HitRecord> {
        let closest_so_far = hit_range.max;
        let mut hit_anything = false;
        let mut record = HitRecord::default();

        for obj in &self.objects {
            if let Some((closest_so_far, normal, mat)) =
                obj.hit(ray, Interval::new(hit_range.min, closest_so_far))
            {
                hit_anything = true;
                record.point = ray.at(closest_so_far);
                record.set_face_normal(ray, normal);
                record.material = mat;
            }
        }

        if !hit_anything {
            return None;
        }

        Some(record)
    }
}
