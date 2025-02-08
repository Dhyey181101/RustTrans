
use std::f64::consts::PI;

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
            return self.hi + PI;
        }
        self.hi - PI
    }

    fn complement(&self) -> GeoS1Interval {
        if self.lo == self.hi {
            return geo_s1_full_interval();
        }
        GeoS1Interval { lo: self.hi, hi: self.lo }
    }

    fn center(&self) -> f64 {
        let c = 0.5 * (self.lo + self.hi);
        if !self.is_inverted() {
            return c;
        }
        if c <= 0.0 {
            return c + PI;
        }
        c - PI
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }
}

fn geo_s1_full_interval() -> GeoS1Interval {
    GeoS1Interval { lo: -PI, hi: PI }
}
