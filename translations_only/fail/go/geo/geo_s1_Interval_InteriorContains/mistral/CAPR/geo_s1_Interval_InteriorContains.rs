

use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn interior_contains(i: &GeoS1Interval, p: &mut f64) -> bool {
    if *p == -f64::consts::PI {
        *p = f64::consts::PI;
    }
    if i.is_inverted() {
        return *p > i.lo || *p < i.hi;
    }
    (*p > i.lo && *p < i.hi) || i.is_full()
}

impl GeoS1Interval {
    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_full(&self) -> bool {
        self.lo == -f64::consts::PI && self.hi == f64::consts::PI
    }
}

