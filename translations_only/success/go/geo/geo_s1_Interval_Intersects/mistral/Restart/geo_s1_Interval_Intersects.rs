
use std::f64;

pub struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    pub fn intersects(&self, oi: &GeoS1Interval) -> bool {
        if self.is_empty() || oi.is_empty() {
            return false;
        }
        if self.is_inverted() {
            return oi.is_inverted() || oi.lo <= self.hi || oi.hi >= self.lo;
        }
        if oi.is_inverted() {
            return oi.lo <= self.hi || oi.hi >= self.lo;
        }
        oi.lo <= self.hi && oi.hi >= self.lo
    }

    pub fn is_empty(&self) -> bool {
        (self.lo == f64::consts::PI) && (self.hi == -f64::consts::PI)
    }

    pub fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }
}
