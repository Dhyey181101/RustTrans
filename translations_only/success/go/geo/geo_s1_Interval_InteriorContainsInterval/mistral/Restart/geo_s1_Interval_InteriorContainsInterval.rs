
use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        (self.lo == f64::consts::PI) && (self.hi == -f64::consts::PI)
    }

    fn is_full(&self) -> bool {
        (self.lo == -f64::consts::PI) && (self.hi == f64::consts::PI)
    }
}

fn interior_contains_interval(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if i.is_inverted() {
        if oi.is_inverted() {
            (oi.lo > i.lo && oi.hi < i.hi) || oi.is_empty()
        } else {
            oi.lo > i.lo || oi.hi < i.hi
        }
    } else if oi.is_inverted() {
        i.is_full() || oi.is_empty()
    } else {
        (oi.lo > i.lo && oi.hi < i.hi) || i.is_full()
    }
}
