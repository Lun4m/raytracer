use std::ops::{Add, Mul};

pub fn lerp<T>(x: T, y: T, alpha: f64) -> T
where
    T: Add<Output = T>,
    f64: Mul<T, Output = T>,
{
    (1.0 - alpha) * x + alpha * y
}
