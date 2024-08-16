use std::{f64::INFINITY, sync::Arc};

use crate::{
    interval::Interval,
    material::ArcMaterial,
    ray::Ray,
    vector::{dot, Vec3},
    volumes::BoundingBox,
};

pub type ArcHittable = Arc<dyn Hittable + Send + Sync>;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> BoundingBox;
}

pub struct HittableList {
    pub objects: Vec<ArcHittable>,
    bbox: BoundingBox,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: BoundingBox::default(),
        }
    }

    pub fn from_vec(objects: Vec<ArcHittable>) -> Self {
        let bbox = objects.iter().fold(BoundingBox::default(), |bbox, obj| {
            BoundingBox::from_boxes(bbox, obj.bounding_box())
        });

        Self { objects, bbox }
    }

    pub fn add(&mut self, obj: ArcHittable) {
        self.bbox = BoundingBox::from_boxes(self.bbox.clone(), obj.bounding_box());
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, mut interval: Interval) -> Option<HitRecord> {
        let mut record = None;

        for obj in self.objects.iter() {
            if let Some(r) = obj.hit(ray, interval) {
                interval.max = r.distance;
                record = Some(r);
            }
        }

        record
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}

// TODO: I'm not 100% sure why we need this, can't we simply place the object
// somewhere else?
// edit: Oooh, maybe we use this to reuse the same object multiple times?
pub struct Translate {
    offset: Vec3,
    object: ArcHittable,
    bbox: BoundingBox,
}

impl Translate {
    pub fn new(object: ArcHittable, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            offset,
            object,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let offset_ray = Ray {
            origin: ray.origin - self.offset,
            ..*ray
        };

        if let Some(mut record) = self.object.hit(&offset_ray, interval) {
            record.point += self.offset;
            return Some(record);
        }

        None
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}

fn lerp<T: Into<f64>>(alpha: T, first: f64, second: f64) -> f64 {
    let a = alpha.into();
    a * first + (1.0 - a) * second
}

pub struct RotateY {
    cos_theta: f64,
    sin_theta: f64,
    object: ArcHittable,
    bbox: BoundingBox,
}

impl RotateY {
    pub fn new(object: ArcHittable, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = lerp(i, bbox.x.min, bbox.x.max);
                    let y = lerp(j, bbox.y.min, bbox.y.max);
                    let z = lerp(k, bbox.z.min, bbox.z.max);

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            cos_theta,
            sin_theta,
            object,
            bbox,
        }
    }

    fn rotate(&self, vec: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * vec.x - self.sin_theta * vec.z,
            vec.y,
            self.sin_theta * vec.x + self.cos_theta * vec.z,
        )
    }
    fn rotate_neg(&self, vec: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * vec.x + self.sin_theta * vec.z,
            vec.y,
            -self.sin_theta * vec.x + self.cos_theta * vec.z,
        )
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let origin = self.rotate(ray.origin);
        let direction = self.rotate(ray.direction);
        let rotated = Ray::new(origin, direction, ray.time);

        if let Some(mut record) = self.object.hit(&rotated, interval) {
            record.point = self.rotate_neg(record.point);
            record.normal = self.rotate_neg(record.normal);
            return Some(record);
        }

        None
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}

/// Struct that keeps track of the hit point, the normal vector at that point,
/// the material of the hit object
pub struct HitRecord {
    pub distance: f64,
    pub point: Vec3,
    // Surface coordinates of the ray-object hit point
    pub uv: (f64, f64),
    pub normal: Vec3,
    pub material: ArcMaterial,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        normal: Vec3,
        uv: (f64, f64),
        distance: f64,
        material: ArcMaterial,
    ) -> Self {
        let point = ray.at(distance);
        let front_face = dot(ray.direction, normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            distance,
            point,
            uv,
            normal,
            material,
            front_face,
        }
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for HitRecord {}

impl PartialOrd for HitRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HitRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .expect("Should not compare NaNs")
    }
}
