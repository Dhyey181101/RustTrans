
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Box<Self> {
        Box::new(Self { lo, hi })
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }
}

fn interior_contains_interval(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if i.is_inverted() {
        if oi.is_inverted() {
            return (oi.lo > i.lo && oi.hi < i.hi) || oi.is_empty();
        }
        return oi.lo > i.lo || oi.hi < i.hi;
    }
    if oi.is_inverted() {
        return i.is_full() || oi.is_empty();
    }
    (oi.lo > i.lo && oi.hi < i.hi) || i.is_full()
}
