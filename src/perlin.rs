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
        // TODO: is there a smarter way to write this stuff?

        let u = point.x - (point.x).floor();
        let v = point.y - (point.y).floor();
        let w = point.z - (point.z).floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = (i + di as i32) & 255;
                    let y = (j + dj as i32) & 255;
                    let z = (k + dk as i32) & 255;
                    c[di][dj][dk] = self.floats[self.perm_x[x as usize]
                        ^ self.perm_y[y as usize]
                        ^ self.perm_z[z as usize]]
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // TODO: is there a smarter way to write this stuff?
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
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
