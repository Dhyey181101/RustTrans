
use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

fn fast_contains(i: &GeoS1Interval, p: f64) -> bool {
    if i.is_inverted() {
        return (p >= i.lo || p <= i.hi) && !i.is_empty();
    }
    p >= i.lo && p <= i.hi
}

impl GeoS1Interval {
    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo == f64::consts::PI && self.hi == -f64::consts::PI
    }
}
