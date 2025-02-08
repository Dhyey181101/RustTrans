
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn add_point(&self, p: f64) -> Interval {
        if p.abs() > PI {
            return *self;
        }
        let mut p = p;
        if p == -PI {
            p = PI;
        }
        if self.fast_contains(p) {
            return *self;
        }
        if self.is_empty() {
            return Interval { lo: p, hi: p };
        }
        if positive_distance(p, self.lo) < positive_distance(self.hi, p) {
            return Interval { lo: p, hi: self.hi };
        }
        Interval { lo: self.lo, hi: p }
    }

    pub fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            (p >= self.lo || p <= self.hi) && !self.is_empty()
        } else {
            p >= self.lo && p <= self.hi
        }
    }

    pub fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    pub fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }
}

fn positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + PI) - (a - PI)
    }
}
