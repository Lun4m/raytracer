use core::panic;
use std::{ops, sync::Arc};

use crate::{
    hittables::{ArcHittable, HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vector::Vec3,
};

// Axis Aligned Bounding Box
#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
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
    const DELTA: f64 = 0.00001;

    pub fn _new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x: x.pad(Self::DELTA),
            y: y.pad(Self::DELTA),
            z: z.pad(Self::DELTA),
        }
    }

    pub fn from_extrema(a: Vec3, b: Vec3) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x)).pad(Self::DELTA);
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y)).pad(Self::DELTA);
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z)).pad(Self::DELTA);
        Self { x, y, z }
    }

    pub fn from_boxes(a: Self, b: Self) -> Self {
        let x = Interval::from_intervals(a.x, b.x).pad(Self::DELTA);
        let y = Interval::from_intervals(a.y, b.y).pad(Self::DELTA);
        let z = Interval::from_intervals(a.z, b.z).pad(Self::DELTA);
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

impl ops::Index<usize> for BoundingBox {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Cannot index above 2"),
        }
    }
}

impl ops::Add<Vec3> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Vec3) -> Self::Output {
        BoundingBox::_new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<BoundingBox> for Vec3 {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        BoundingBox::_new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Bounding Volume Hierarhcy
pub struct BvhNode {
    left: ArcHittable,
    right: Option<ArcHittable>,
    bbox: BoundingBox,
}

impl BvhNode {
    pub fn new(objects: &mut [ArcHittable], start: usize, end: usize) -> Self {
        let bbox = objects[start..end]
            .iter()
            .fold(BoundingBox::default(), |bbox, obj| {
                BoundingBox::from_boxes(bbox, obj.bounding_box())
            });

        let axis = bbox.longest_axis();
        let span = end - start;

        let (left, right) = match span {
            1 => (objects[start].clone(), None),
            2 => (objects[start].clone(), Some(objects[start + 1].clone())),
            _ => {
                objects[start..end].sort_by(|a, b| {
                    let a_interval = a.bounding_box()[axis];
                    let b_interval = b.bounding_box()[axis];

                    a_interval.min.partial_cmp(&b_interval.min).unwrap()
                });

                let mid = start + span / 2;
                let left = Arc::new(Self::new(objects, start, mid));
                let right = Arc::new(Self::new(objects, mid, end));
                (left as ArcHittable, Some(right as ArcHittable))
            }
        };

        Self { left, right, bbox }
    }
}

impl From<HittableList> for BvhNode {
    fn from(mut value: HittableList) -> Self {
        let len = value.objects.len();
        Self::new(&mut value.objects, 0, len)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, mut interval: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval) {
            return None;
        }

        let mut record = None;
        if let Some(left_obj) = self.left.hit(ray, interval) {
            interval.max = left_obj.distance;
            record = Some(left_obj);
        };

        if let Some(node) = &self.right {
            if let Some(right_obj) = node.hit(ray, interval) {
                record = Some(right_obj);
            };
        }

        record
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}
