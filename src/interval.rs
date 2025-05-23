pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Self {
        Self { min: a, max: b }
    }

    pub fn empty() -> Self {
        Self { min: f64::INFINITY, max: f64::NEG_INFINITY } // empty interval
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }   

    // define empty and universe intervals
    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}

