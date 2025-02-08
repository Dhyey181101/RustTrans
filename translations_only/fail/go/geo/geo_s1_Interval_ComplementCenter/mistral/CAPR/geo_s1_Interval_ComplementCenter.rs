

use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn complement_center(&self) -> f64 {
        if self.lo != self.hi {
            return self.complement().center();
        }
        if self.hi <= 0.0 {
            return self.hi + f64::consts::PI;
        }
        return self.hi - f64::consts::PI;
    }

    fn complement(&self) -> GeoS1Interval {
        if self.lo == self.hi {
            return GeoS1FullInterval;
        }
        GeoS1Interval { hi: self.hi, lo: self.lo }
    }
}

const GeoS1FullInterval: GeoS1Interval = GeoS1Interval { lo: -f64::consts::PI, hi: f64::consts::PI };

impl GeoS1Interval {
    fn center(&self) -> f64 {
        let c = 0.5 * (self.lo + self.hi);
        if !self.is_inverted() {
            return c;
        }
        if c <= 0.0 {
            return c + f64::consts::PI;
        }
        return c - f64::consts::PI;
    }

    fn is_inverted(&self) -> bool {
        return self.lo > self.hi;
    }
}

