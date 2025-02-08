
use std::f64::consts::PI;

#[derive(Clone)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    fn add_point(&self, p: f64) -> GeoS1Interval {
        if p.abs() > PI {
            return self.clone();
        }
        let mut p = p;
        if p == -PI {
            p = PI;
        }
        if fast_contains(self, p) {
            return self.clone();
        }
        if is_empty(self) {
            return GeoS1Interval::new(p, p);
        }
        if geo_s1_positive_distance(p, self.lo) < geo_s1_positive_distance(self.hi, p) {
            return GeoS1Interval::new(p, self.hi);
        }
        GeoS1Interval::new(self.lo, p)
    }
}

fn fast_contains(interval: &GeoS1Interval, p: f64) -> bool {
    if is_inverted(interval) {
        return (p >= interval.lo || p <= interval.hi) && !is_empty(interval);
    }
    p >= interval.lo && p <= interval.hi
}

fn is_inverted(interval: &GeoS1Interval) -> bool {
    interval.lo > interval.hi
}

fn is_empty(interval: &GeoS1Interval) -> bool {
    interval.lo == PI && interval.hi == -PI
}

fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        return d;
    }
    (b + PI) - (a - PI)
}

fn main() {}
