use std::sync::Arc;

use crate::{
    hittables::{HitRecord, Hittable},
    ray::Ray,
    volumes::BoundingBox,
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

impl Hittable for World {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // I don't care if this is slower
        // NOTE: this is completely useless with BVH?
        // self.objects should be the BVH tree?
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray))
            .min_by(|x, y| x.cmp(y))
    }

    fn bounding_box(&self) -> BoundingBox {
        todo!()
    }
}
