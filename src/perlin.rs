use crate::{
    random,
    utils::lerp,
    vector::{dot, unit_vector, Vec3},
};

pub struct Perlin {
    // point_count: usize,
    // floats: Vec<f64>,
    perm: Vec<usize>,
    rand_vecs: Vec<Vec3>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        // let rand_float = (0..point_count).map(|_| random::float()).collect();
        let rand_vecs = (0..point_count)
            .map(|_| unit_vector(Vec3::random_min_max(-1.0, 1.0)))
            .collect();

        Self {
            // floats: rand_float,
            rand_vecs,
            perm: Self::generate_perm(point_count),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let u = point.x - (point.x).floor();
        let v = point.y - (point.y).floor();
        let w = point.z - (point.z).floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = i + di as i32;
                    let y = j + dj as i32;
                    let z = k + dk as i32;
                    c[di][dj][dk] = self.rand_vecs[self.perm[(x & 255) as usize]
                        ^ self.perm[(y & 255) as usize]
                        ^ self.perm[(z & 255) as usize]]
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    pub fn turbulence(&self, point: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(point);
            weight *= 0.5;
        }

        accum.abs()
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermitian smoothing
        let u = Self::hermitian_fade(u);
        let v = Self::hermitian_fade(v);
        let w = Self::hermitian_fade(w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                    accum += lerp(1.0 - u, u, i as f64)
                        * lerp(1.0 - v, v, j as f64)
                        * lerp(1.0 - w, w, k as f64)
                        * dot(c[i][j][k], weight);
                }
            }
        }

        accum
    }

    fn hermitian_fade(x: f64) -> f64 {
        x * x * (3.0 - 2.0 * x)
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
