
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Box<Self> {
        Box::new(Self { lo, hi })
    }

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }
}

fn interior_intersects(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if i.is_empty() || oi.is_empty() || i.lo == i.hi {
        return false;
    }
    if i.is_inverted() {
        return oi.is_inverted() || oi.lo < i.hi || oi.hi > i.lo;
    }
    if oi.is_inverted() {
        return oi.lo < i.hi || oi.hi > i.lo;
    }
    (oi.lo < i.hi && oi.hi > i.lo) || i.is_full()
}
