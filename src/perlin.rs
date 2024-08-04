use crate::{random, vector::Vec3};

pub struct Perlin {
    // point_count: usize,
    floats: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: Option<usize>) -> Self {
        let point_count = point_count.unwrap_or(256);
        let rand_float = (0..point_count).map(|_| random::float()).collect();

        Self {
            floats: rand_float,
            perm_x: Self::generate_perm(point_count),
            perm_y: Self::generate_perm(point_count),
            perm_z: Self::generate_perm(point_count),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let i = ((4.0 * point.x) as i32 & 255) as usize;
        let j = ((4.0 * point.y) as i32 & 255) as usize;
        let k = ((4.0 * point.z) as i32 & 255) as usize;

        self.floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn generate_perm(point_count: usize) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..point_count).collect();
        // use rand::seq::SliceRandom;
        // let mut rng = rand::thread_rng();
        // perm.shuffle(&mut rng);

        for i in (1..point_count).rev() {
            perm.swap(i, random::usize(0, i));
        }

        perm
    }
}
