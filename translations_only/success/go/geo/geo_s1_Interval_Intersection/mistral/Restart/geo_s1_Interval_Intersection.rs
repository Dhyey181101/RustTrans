
use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
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
        l + 2.0 * f64::consts::PI
    }
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval {
        lo: std::f64::consts::PI,
        hi: -std::f64::consts::PI,
    }
}

fn geo_s1_interval_intersection(
    i: &GeoS1Interval,
    oi: &GeoS1Interval,
) -> GeoS1Interval {
    if oi.is_empty() {
        GeoS1Interval { lo: f64::NAN, hi: f64::NAN }
    } else {
        GeoS1Interval {
            lo: f64::max(i.lo, oi.lo),
            hi: f64::min(i.hi, oi.hi),
        }
    }
}
