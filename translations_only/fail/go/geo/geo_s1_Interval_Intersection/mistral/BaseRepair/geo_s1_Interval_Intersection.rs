

use std::f64;

const TWO_PI: f64 = 2.0 * (f64::consts::PI);

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    fn is_empty(&self) -> bool {
        self.lo == f64::consts::PI && self.hi == -f64::consts::PI
    }

    fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            return (p >= self.lo || p <= self.hi) && !self.is_empty();
        }
        p >= self.lo && p <= self.hi
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        let l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l + TWO_PI
    }
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval::new(f64::consts::PI, -f64::consts::PI)
}

fn geo_s1_interval_intersection(i: &GeoS1Interval, oi: &GeoS1Interval) -> GeoS1Interval {
    if oi.is_empty() {
        return geo_s1_empty_interval();
    }
    if i.fast_contains(oi.lo) {
        if i.fast_contains(oi.hi) {
            if oi.length() < i.length() {
                return GeoS1Interval::new(oi.lo, oi.hi);
            }
            return GeoS1Interval::new(i.lo, i.hi);
        }
        return GeoS1Interval::new(oi.lo, i.hi);
    }
    if i.fast_contains(oi.hi) {
        return GeoS1Interval::new(i.lo, oi.hi);
    }

    if oi.fast_contains(i.lo) {
        return GeoS1Interval::new(i.lo, i.hi);
    }
    geo_s1_empty_interval()
}

