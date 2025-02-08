

use std::f64;

struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn contains(&self, p: f64) -> bool {
        let p = if p == -f64::consts::PI {
            f64::consts::PI
        } else {
            p
        };
        self.fast_contains(p)
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
        self.lo.is_nan() && self.hi.is_nan()
    }
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn contains(interval: &GeoS1Interval, p: f64) -> bool {
    let p = clamp(p, -f64::consts::PI, f64::consts::PI);
    interval.fast_contains(p)
}

