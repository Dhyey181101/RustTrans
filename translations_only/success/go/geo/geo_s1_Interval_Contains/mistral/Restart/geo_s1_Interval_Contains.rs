

use std::f64;

pub struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    pub fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    pub fn contains(&self, p: f64) -> bool {
        if p == -f64::consts::PI {
            let p_adjusted = f64::consts::PI;
            self.fast_contains(p_adjusted)
        } else {
            self.fast_contains(p)
        }
    }

    fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            if p == -f64::consts::PI {
                let p_adjusted = f64::consts::PI;
                (p_adjusted >= self.lo || p_adjusted <= self.hi) && !self.is_empty()
            } else {
                (p >= self.lo || p <= self.hi) && !self.is_empty()
            }
        } else {
            p >= self.lo && p <= self.hi
        }
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn is_empty(&self) -> bool {
        self.lo == f64::consts::PI && self.hi == -f64::consts::PI
    }
}

fn geo_s1_interval_contains(interval: &GeoS1Interval, p: f64) -> bool {
    if p == -f64::consts::PI {
        let p_adjusted = f64::consts::PI;
        interval.fast_contains(p_adjusted)
    } else {
        interval.fast_contains(p)
    }
}

