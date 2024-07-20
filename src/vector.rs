use std::ops::{self};

use rand::{random, thread_rng, Rng};
use rand_distr::StandardNormal;

pub fn random_in_interval(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

// TODO: should not derive Copy here lol
#[derive(Debug, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub const EPS: f64 = 1e-8;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self::new(random(), random(), random())
    }

    pub fn random_min_max(min: f64, max: f64) -> Self {
        Self::new(
            random_in_interval(min, max),
            random_in_interval(min, max),
            random_in_interval(min, max),
        )
    }

    pub fn random_normal() -> Self {
        Self::new(
            thread_rng().sample(StandardNormal),
            thread_rng().sample(StandardNormal),
            thread_rng().sample(StandardNormal),
        )
    }

    pub fn random_normal_xy() -> Self {
        Self::new(
            thread_rng().sample(StandardNormal),
            thread_rng().sample(StandardNormal),
            0.0,
        )
    }

    // uniformely sample a point in a unit sphere
    // TODO: this rejection method is so slow!
    // pub fn random_in_unit_sphere() -> Self {
    //     loop {
    //         // Sample in unit cube
    //         let v = Self::random_min_max(-1.0, 1.0);
    //         // Reject if outside unit sphere
    //         if v.len_squared() < 1.0 {
    //             return unit_vector(v);
    //         }
    //     }
    // }

    // uniformely sample a point in a unit sphere
    // This one is 20% faster for me compared to the rejection method
    pub fn random_in_unit_sphere() -> Self {
        let u = random::<f64>();
        u.cbrt() * unit_vector(&Self::random_normal())
    }

    // uniformely sample a point in a unit circle on the xy plane
    pub fn random_in_unit_disk() -> Vec3 {
        let u = random::<f64>();
        u.sqrt() * unit_vector(&Self::random_normal_xy())
    }

    pub fn random_in_unit_cube() -> Self {
        unit_vector(&Self::random_min_max(-1.0, 1.0))
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_in_unit_sphere();
        let sign = dot(&on_unit_sphere, normal).signum();
        sign * on_unit_sphere
    }

    // pub fn normalize(&mut self) {
    //     *self / self.len();
    // }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        (self.x.abs() < EPS) && (self.x.abs() < EPS) && (self.x.abs() < EPS)
    }
}

// TODO: use macro to register these traits
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> ops::Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'a Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl<'a> ops::Add<f64> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<i32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl<'a> ops::Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl ops::Mul<&Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<i32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl core::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Self::default();
        for item in iter {
            result += item;
        }
        result
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.len()
}
