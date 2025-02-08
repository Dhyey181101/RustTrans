
use std::f64::consts::PI;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn add_point(&self, p: f64) -> Box<GeoS1Interval> {
        if p.abs() > PI {
            return Box::new(GeoS1Interval { lo: self.lo, hi: self.hi });
        }
        let mut p = p;
        if p == -PI {
            p = PI;
        }
        if self.fast_contains(p) {
            return Box::new(GeoS1Interval { lo: self.lo, hi: self.hi });
        }
        if self.is_empty() {
            return Box::new(GeoS1Interval { lo: p, hi: p });
        }
        if geo_s1_positive_distance(p, self.lo) < geo_s1_positive_distance(self.hi, p) {
            return Box::new(GeoS1Interval { lo: p, hi: self.hi });
        }
        Box::new(GeoS1Interval { lo: self.lo, hi: p })
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

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }
}

fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        return d;
    }
    (b + PI) - (a - PI)
}
