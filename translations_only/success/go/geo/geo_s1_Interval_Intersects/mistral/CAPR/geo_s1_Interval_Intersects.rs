
use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn is_empty(&self) -> bool {
        self.lo == f64::consts::PI && self.hi == -f64::consts::PI
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }
}

fn intersects(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if i.is_empty() || oi.is_empty() {
        return false;
    }
    if i.is_inverted() {
        return oi.is_inverted() || oi.lo <= i.hi || oi.hi >= i.lo;
    }
    if oi.is_inverted() {
        return oi.lo <= i.hi || oi.hi >= i.lo;
    }
    oi.lo <= i.hi && oi.hi >= i.lo
}
