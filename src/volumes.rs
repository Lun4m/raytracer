// TODO: this whole module needs to be rewritten in a sane way
use std::{cmp::min, ops::Index, sync::Arc};

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vector::Vec3,
    world::World,
};

// Axis Aligned Bounding Box
#[derive(Clone, Debug)]
pub struct BoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Default for BoundingBox {
    /// Returns an empty bounding box
    fn default() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }
}

impl BoundingBox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Vec3, b: Vec3) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Self { x, y, z }
    }

    pub fn from_boxes(a: Self, b: Self) -> Self {
        let x = Interval::from_intervals(&a.x, &b.x);
        let y = Interval::from_intervals(&a.y, &b.y);
        let z = Interval::from_intervals(&a.z, &b.z);
        Self { x, y, z }
    }
    pub fn longest_axis(&self) -> usize {
        if self.x.span() > self.y.span() {
            if self.x.span() > self.z.span() {
                return 0;
            }
            return 2;
        }
        if self.y.span() > self.z.span() {
            return 1;
        }

        2
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        // TODO: can this be simplified?
        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / ray.direction[axis];

            // Bounding box - Ray intersections
            let t0 = (ax.min - ray.origin[axis]) * adinv;
            let t1 = (ax.max - ray.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                }
                if t1 < ray_t.max {
                    ray_t.max = t1
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1
                }
                if t0 < ray_t.max {
                    ray_t.max = t0
                }
            }
        }
        if ray_t.max <= ray_t.min {
            return false;
        }
        true
    }
}

impl Index<usize> for BoundingBox {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 1 {
            return &self.y;
        }
        if index == 2 {
            return &self.z;
        }
        &self.x
    }
}

// Bounding Volume Hierarhcy
pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: BoundingBox,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> Self {
        let bbox = objects
            .iter()
            .take(end)
            .skip(start)
            .fold(BoundingBox::default(), |bbox, obj| {
                BoundingBox::from_boxes(bbox, obj.bounding_box())
            });

        let axis = bbox.longest_axis();
        let span = end - start;

        let (left, right) = match span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                objects[start..end].sort_by(|a, b| {
                    let a_interval = &a.bounding_box()[axis];
                    let b_interval = &b.bounding_box()[axis];

                    a_interval.min.partial_cmp(&b_interval.min).unwrap()
                });

                let mid = start + span / 2;
                let left = Arc::new(Self::new(objects, start, mid));
                let right = Arc::new(Self::new(objects, mid, end));
                (
                    left as Arc<dyn Hittable + Send + Sync>,
                    right as Arc<dyn Hittable + Send + Sync>,
                )
            }
        };

        Self { left, right, bbox }
    }

    pub fn from_world(mut world: World) -> Self {
        let len = world.objects.len();
        Self::new(&mut world.objects, 0, len)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        if !self.bbox.hit(ray, Interval::_positive()) {
            return None;
        }

        // TODO: these need the interval so
        // boxes down the tree can filter out
        // rays that do not intersect?
        let hit_left = self.left.hit(ray);
        let hit_right = self.right.hit(ray);

        match (hit_left, hit_right) {
            (Some(a), Some(b)) => Some(min(a, b)),
            (None, b) => b,
            (a, None) => a,
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}
