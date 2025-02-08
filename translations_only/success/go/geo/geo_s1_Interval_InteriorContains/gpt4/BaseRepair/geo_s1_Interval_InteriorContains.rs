
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Box<Self> {
        Box::new(Self { lo, hi })
    }

    fn interior_contains(&self, p: f64) -> bool {
        let mut p = p;
        if p == -PI {
            p = PI;
        }
        if self.is_inverted() {
            return p > self.lo || p < self.hi;
        }
        (p > self.lo && p < self.hi) || self.is_full()
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }
}

