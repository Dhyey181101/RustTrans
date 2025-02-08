
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn intersection(&self, other: &Interval) -> Interval {
        if other.is_empty() {
            return Interval::empty();
        }
        if self.fast_contains(other.lo) {
            if self.fast_contains(other.hi) {
                if other.length() < self.length() {
                    return *other;
                }
                return *self;
            }
            return Interval {
                lo: other.lo,
                hi: self.hi,
            };
        }
        if self.fast_contains(other.hi) {
            return Interval {
                lo: self.lo,
                hi: other.hi,
            };
        }
        if other.fast_contains(self.lo) {
            return *self;
        }
        Interval::empty()
    }

    pub fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    pub fn empty() -> Interval {
        Interval {
            lo: PI,
            hi: -PI,
        }
    }

    pub fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            return (p >= self.lo || p <= self.hi) && !self.is_empty();
        }
        p >= self.lo && p <= self.hi
    }

    pub fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    pub fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        let l = l + 2.0 * PI;
        if l > 0.0 {
            return l;
        }
        -1.0
    }
}
