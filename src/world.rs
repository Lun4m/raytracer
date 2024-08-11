use std::sync::Arc;

use crate::{
    hittables::{ArcHittable, HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    volumes::BoundingBox,
};

pub struct World {
    pub objects: Vec<ArcHittable>,
    bbox: BoundingBox,
}

impl World {
    pub fn _new() -> Self {
        World {
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

impl Hittable for World {
    fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<HitRecord> {
        let mut record = None;

        for obj in self.objects.iter() {
            if let Some(r) = obj.hit(ray, interval) {
                interval.max = r.distance;
                record = Some(r);
            }
        }

        // record

        // self.objects.iter().for_each(|obj| {
        //     if let Some(r) = obj.hit(ray, interval) {
        //         interval.max = r.distance;
        //         record = Some(r);
        //     }
        // });

        record
        // .filter_map(|obj| obj.hit(ray))
        // .min_by(|x, y| x.cmp(y))
    }

    fn bounding_box(&self) -> BoundingBox {
        todo!()
    }
}
