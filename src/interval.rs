use std::f64::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn _empty() -> Self {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub fn _universe() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn _positive() -> Self {
        Interval {
            min: 0.001,
            max: INFINITY,
        }
    }

    pub fn _negative() -> Self {
        Interval {
            min: -INFINITY,
            max: 0.0,
        }
    }

    // TODO: check that these methods are correct
    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn _surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }

        x
    }
}
