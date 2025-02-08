

use std::f64;

const PI: f64 = std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn contains_interval(i: &GeoS1Interval, oi: &GeoS1Interval) -> bool {
    if i.is_inverted() {
        if oi.is_inverted() {
            oi.lo >= i.lo && oi.hi <= i.hi
        } else {
            (oi.lo >= i.lo || oi.hi <= i.hi) && !i.is_empty()
        }
    } else {
        if oi.is_inverted() {
            i.is_full() || oi.is_empty()
        } else {
            oi.lo >= i.lo && oi.hi <= i.hi
        }
    }
}

impl GeoS1Interval {
    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo.is_nan() && self.hi.is_nan()
    }

    fn is_full(&self) -> bool {
        self.lo == -PI && self.hi == PI
    }
}

