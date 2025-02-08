
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn project(&self, p: f64) -> f64 {
        if p == -PI {
            PI
        } else if self.fast_contains(p) {
            p
        } else {
            let dlo = positive_distance(p, self.lo);
            let dhi = positive_distance(self.hi, p);
            if dlo < dhi {
                self.lo
            } else {
                self.hi
            }
        }
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
