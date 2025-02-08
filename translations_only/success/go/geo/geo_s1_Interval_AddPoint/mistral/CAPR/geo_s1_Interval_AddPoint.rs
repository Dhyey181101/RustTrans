

use std::f64;

const PI: f64 = std::f64::consts::PI;

fn geo_s1_positive_distance(a: f64, b: f64) -> f64 {
    let d = b - a;
    if d >= 0.0 {
        d
    } else {
        (b + PI) - (a - PI)
    }
}

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn add_point(&self, p: f64) -> GeoS1Interval {
        let mut p = if p.abs() > PI { self.lo } else { p };
        if p == -PI {
            p = PI;
        }
        let mut result = GeoS1Interval { lo: self.lo, hi: self.hi };
        if result.fast_contains(p.clone()) {
            return result;
        }
        if result.is_empty() {
            result.lo = p;
            result.hi = p;
            return result;
        }
        if geo_s1_positive_distance(p.clone(), self.lo)
            < geo_s1_positive_distance(self.hi.clone(), p)
        {
            result.lo = p;
            result.hi = self.hi;
        } else {
            result.lo = self.lo;
            result.hi = p;
        }
        result
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

