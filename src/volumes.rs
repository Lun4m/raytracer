use std::{cmp::min, ops::Index, rc::Rc, sync::Arc};

use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    random,
    ray::Ray,
    vector::Vec3,
    world::World,
};

// Axis Aligned Bounding Box
#[derive(Clone)]
pub struct BoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Default for BoundingBox {
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
        // let x = if a.x <= b.x {
        //     Interval::new(a.x, b.x)
        // } else {
        //     Interval::new(b.x, a.x)
        // };
        // let y = if a.y <= b.y {
        //     Interval::new(a.y, b.y)
        // } else {
        //     Interval::new(b.y, a.y)
        // };
        // let z = if a.z <= b.z {
        //     Interval::new(a.z, b.z)
        // } else {
        //     Interval::new(b.z, a.z)
        // };
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Self { x, y, z }
    }

    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        let x = Interval::from_intervals(&a.x, &b.x);
        let y = Interval::from_intervals(&a.y, &b.y);
        let z = Interval::from_intervals(&a.z, &b.z);
        Self { x, y, z }
    }

    pub fn from_box(a: &Self) -> Self {
        Self {
            x: a.x.clone(),
            y: a.y.clone(),
            z: a.z.clone(),
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / ray.direction[axis]; // ray_dir[axis]

            // Bounding box - Ray intersection
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

// pub struct BvhTree {
//     objects:
// }

// Bounding Volume Hierarhcy
pub struct BvhNode {
    // TODO: leaf nodes need to be the objects!
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    // val: Option<usize>,
    bbox: BoundingBox,
}

impl BvhNode {
    pub fn new(objects: &Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        let axis = random::usize(0, 2);
        let span = end - start;

        match span {
            1 => Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: BoundingBox::from_box(objects[start].bounding_box()),
            },
            2 => Self {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox: BoundingBox::from_boxes(
                    objects[start].bounding_box(),
                    objects[start + 1].bounding_box(),
                ),
            },
            _ => {
                // sort vec??
                //
                //
                let mid = start + span / 2;
                let left = Self::new(objects, start, mid);
                let right = Self::new(objects, mid, end);
                let bbox = BoundingBox::from_boxes(left.bounding_box(), right.bounding_box());

                Self {
                    left: Arc::new(left),
                    right: Arc::new(right),
                    bbox,
                }
            }
        }
    }

    pub fn from_world(world: World) -> Self {
        Self::new(&world.objects, 0, world.objects.len())
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let hit_right = self.right.hit(ray, ray_t);

        match (hit_left, hit_right) {
            (Some(a), Some(b)) => Some(min(a, b)),
            (None, b) => b,
            (a, None) => a,
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bbox
    }
}
