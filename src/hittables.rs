use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{dot, Vec3},
    volumes::BoundingBox,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &BoundingBox;
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

// #[derive(Ord)]
impl<'a> HitRecord<'a> {
    /// Sets hit record of normal vector,
    /// so that we can distinguish if we are inside or outside an object
    /// outward_normal assumed to have unit norm
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

impl Eq for HitRecord<'_> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for HitRecord<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.total_cmp(&other.distance))
    }
}

impl Ord for HitRecord<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}
