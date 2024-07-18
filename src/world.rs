use crate::{hittables::HitRecord, interval::Interval, ray::Ray, sphere::Sphere};

pub struct World {
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn from(objects: Vec<Sphere>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }

    // TODO: refactor this function, it's a bit messy with the default HitRecord
    pub fn hit(&self, ray: &Ray, ray_hit: Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_hit.max;
        let mut temp_record = HitRecord::default();

        for obj in &self.objects {
            if obj.hit(
                ray,
                Interval::new(ray_hit.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                record.update(&temp_record);
            }
        }

        hit_anything
    }
}
