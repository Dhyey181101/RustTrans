
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn complement_center(&self) -> f64 {
        if self.lo != self.hi {
            return self.complement().center();
        }
        // Singleton. The interval just contains a single point.
        if self.hi <= 0.0 {
            return self.hi + PI;
        }
        self.hi - PI
    }

    pub fn complement(&self) -> Interval {
        if self.lo == self.hi {
            // Singleton. The interval just contains a single point.
            return Interval { lo: -PI, hi: PI };
        }
        // Handles empty and full.
        Interval { lo: self.hi, hi: self.lo }
    }

    pub fn full_interval() -> Interval {
        Interval { lo: -PI, hi: PI }
    }

    pub fn center(&self) -> f64 {
        let c = 0.5 * (self.lo + self.hi);
        if !self.is_inverted() {
            return c;
        }
        if c <= 0.0 {
            return c + PI;
        }
        c - PI
    }

    pub fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }
}

