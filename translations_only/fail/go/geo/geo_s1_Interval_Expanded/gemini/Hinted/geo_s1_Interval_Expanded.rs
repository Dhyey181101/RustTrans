
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn expanded(&self, margin: f64) -> Interval {
        let mut result = Interval::from_endpoints(
            self.lo - margin,
            self.hi + margin,
        );
        if result.lo <= -PI {
            result.lo = PI;
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    pub fn from_endpoints(lo: f64, hi: f64) -> Interval {
        let mut i = Interval { lo, hi };
        if lo == -PI && hi != PI {
            i.lo = PI;
        }
        if hi == -PI && lo != PI {
            i.hi = PI;
        }
        i
    }

    pub fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }

    pub fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l + 2.0 * PI
    }

    pub fn full() -> Interval {
        Interval {
            lo: -PI,
            hi: PI,
        }
    }

    pub fn empty() -> Interval {
        Interval {
            lo: PI,
            hi: -PI,
        }
    }
}
