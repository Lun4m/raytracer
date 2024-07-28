use rand::random;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

pub fn float() -> f64 {
    random()
}

pub fn _usize(min: usize, max: usize) -> usize {
    min + (max + 1 - min) * random::<usize>()
}

pub fn normal() -> f64 {
    thread_rng().sample(StandardNormal)
}

pub fn in_interval(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}
