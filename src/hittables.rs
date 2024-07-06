use crate::{
    ray::Ray,
    vector::{dot, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool;
}

#[derive(Debug, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets hit record of normal vector,
    /// so that we can distinguish if we are inside or outside an object
    /// outward_normal assumed to have unit norm
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    fn update(&mut self, other: &HitRecord) {
        self.point = other.point;
        self.normal = other.normal;
        self.t = other.t;
        self.front_face = other.front_face;
    }
}

pub struct HitList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = tmax;
        let mut temp_record = HitRecord::default();

        for obj in &self.objects {
            if obj.hit(ray, tmin, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                record.update(&temp_record);
            }
        }

        hit_anything
    }
}

impl HitList {
    pub fn new() -> Self {
        HitList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}
