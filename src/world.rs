use crate::{
    hittables::{HitRecord, Hittable},
    ray::Ray,
    sphere::Sphere,
};

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
}

impl Hittable for World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // I don't care if this is slower
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray))
            .min_by(|x, y| x.distance.total_cmp(&y.distance))
    }
}
