
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn contains(&self, p: f64) -> bool {
        let mut p = p;
        if p == -PI {
            p = PI;
        }
        self.fast_contains(p)
    }

    fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            (p >= self.lo || p <= self.hi) && !self.is_empty()
        } else {
            p >= self.lo && p <= self.hi
        }
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }
}
