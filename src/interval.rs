use std::f64::INFINITY;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        Self { min, max }
    }

    pub fn empty() -> Self {
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

    pub fn positive() -> Self {
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

    pub fn span(&self) -> f64 {
        self.max - self.min
    }

    // TODO: check that these methods are correct
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
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
    pub fn pad(&self, delta: f64) -> Self {
        if self.span() < delta {
            return self.expand(delta);
        }

        *self
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = 0.5 * delta;
        Self::new(self.min - padding, self.max + padding)
    }
}
