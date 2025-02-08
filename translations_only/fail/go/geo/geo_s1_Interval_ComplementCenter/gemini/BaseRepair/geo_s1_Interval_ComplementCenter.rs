
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    lo: f64,
    hi: f64,
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
        return self.hi - PI;
    }

    pub fn complement(&self) -> Interval {
        if self.lo == self.hi {
            // Singleton. The interval just contains a single point.
            return full_interval();
        }
        // Handles empty and full.
        return Interval { hi: self.hi, lo: self.lo };
    }

    pub fn center(&self) -> f64 {
        let c = 0.5 * (self.lo + self.hi);
        if !self.is_inverted() {
            return c;
        }
        if c <= 0.0 {
            return c + PI;
        }
        return c - PI;
    }

    pub fn is_inverted(&self) -> bool {
        return self.lo > self.hi;
    }
}

pub fn full_interval() -> Interval {
    return Interval { lo: -PI, hi: PI };
}
