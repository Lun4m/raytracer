use std::sync::Arc;

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

    // TODO: do we need this?
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
        Self {
            objects,
            bbox: BoundingBox::default(),
        }
    }

    pub fn add(&mut self, obj: impl Hittable + Send + Sync + 'static) {
        self.bbox = BoundingBox::from_boxes(self.bbox.clone(), obj.bounding_box());
        self.objects.push(Arc::new(obj));
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
        todo!()
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
