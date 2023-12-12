use std::f32::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    const empty: Interval = Interval {
        min: INFINITY,
        max: NEG_INFINITY,
    };
    const universe: Interval = Interval {
        min: NEG_INFINITY,
        max: INFINITY,
    };
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(-1.0, -1.0)
    }
}
