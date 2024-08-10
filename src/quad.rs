use crate::{
    hittables::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{cross, dot, unit_vector, Vec3, EPS},
    volumes::BoundingBox,
};

pub struct Quad {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    d: f64,
    w: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Send + Sync>,
    bbox: BoundingBox,
}

impl Quad {
    #[rustfmt::skip]
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, material: impl Material + Send + Sync + 'static) -> Self {
        // Plane formula:
        //   ax + by + cz = d
        //   origin = (x, y, z)
        //   normal = (a, b, c) = u cross v
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, origin);

        // Vector used to check if the ray intersection point
        // lies inside the quad 
        let w = n / n.len_squared();

        let bbox = Self::set_bbox(origin, u, v);
        Self { origin, u, v, d, w, normal, bbox, material: Box::new(material) }
    }

    fn set_bbox(o: Vec3, u: Vec3, v: Vec3) -> BoundingBox {
        let diag_1 = BoundingBox::from_extrema(o, o + u + v);
        let diag_2 = BoundingBox::from_extrema(o + u, o + v);

        BoundingBox::from_boxes(diag_1, diag_2)
    }

    fn get_uv(&self, intersection: Vec3) -> Option<(f64, f64)> {
        let o_to_hp = intersection - self.origin;

        let alpha = dot(self.w, cross(o_to_hp, self.v));
        let beta = dot(self.w, cross(self.u, o_to_hp));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        Some((alpha, beta))
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        // n dot (r.orig + t * r.dir) = d
        // t = (d - n dot p.orig) / (n dot r.dir)
        let denom = dot(self.normal, ray.direction);

        // Ray parallel to plane
        if denom.abs() < EPS {
            return None;
        }

        let root = (self.d - dot(self.normal, ray.origin)) / denom;
        let uv = self.get_uv(ray.at(root))?;

        Some(HitRecord::new(
            ray,
            self.normal,
            uv,
            root,
            self.material.as_ref(),
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bbox.clone()
    }
}
