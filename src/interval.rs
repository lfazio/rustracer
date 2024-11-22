#[derive(Debug, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn new_enclosing(a: &Interval, b: &Interval) -> Interval {
        // Create the interval tightly enclosing the two input intervals.
        Interval {
            min: if a.min <= b.min { a.min() } else { b.min() },
            max: if a.max >= b.max { a.max() } else { b.max() },
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
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

    pub fn expand(&self, delta: &f64) -> Interval {
        Interval::new(self.min - delta / 2.0, self.max + delta / 2.0)
    }
}

impl std::default::Default for Interval {
    fn default() -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

const UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};
const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
