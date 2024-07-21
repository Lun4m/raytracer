use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    ray::Ray,
    volumes::{BoundingBox, BvhNode},
};

pub struct World {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    bbox: BoundingBox,
}

impl World {
    pub fn _new() -> Self {
        World {
            objects: Vec::new(),
            bbox: BoundingBox::default(),
        }
    }

    pub fn from_vec(objects: Vec<Arc<dyn Hittable + Send + Sync>>) -> Self {
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

impl From<BvhNode> for World {
    fn from(value: BvhNode) -> Self {
        let bbox = value.bounding_box().clone();
        Self {
            objects: vec![Arc::new(value)],
            bbox,
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // I don't care if this is slower
        // TODO: this is completely useless with BVH?
        // self.objects should be the BVH tree?
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray))
            .min_by(|x, y| x.distance.total_cmp(&y.distance))

        // self.objects
        //     .iter()
        //     .scan(ray_t.max, |closest_so_far, obj| {
        //         match obj.hit(ray, &Interval::new(ray_t.min, *closest_so_far)) {
        //             Some(record) => {
        //                 *closest_so_far = record.distance;
        //                 Some(Some(record))
        //             }
        //             None => Some(None),
        //         }
        //     })
        //     .flatten()
        //     .min_by(|x, y| x.distance.total_cmp(&y.distance))
    }

    fn bounding_box(&self) -> BoundingBox {
        todo!()
    }
}
