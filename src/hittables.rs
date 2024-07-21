use crate::{
    material::Material,
    ray::Ray,
    vector::{dot, Vec3},
    volumes::BoundingBox,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
    // TODO: do we need this?
    fn bounding_box(&self) -> BoundingBox;
}

/// Struct that keeps track of the hit point, the normal vector at that point,
/// the material of the hit object
pub struct HitRecord<'a> {
    pub distance: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(ray: &Ray, normal: Vec3, distance: f64, material: &'a dyn Material) -> Self {
        let point = ray.at(distance);
        let front_face = dot(&ray.direction, &normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            distance,
            point,
            normal,
            material,
            front_face,
        }
    }
}

impl PartialEq for HitRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for HitRecord<'_> {}

impl PartialOrd for HitRecord<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HitRecord<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}
