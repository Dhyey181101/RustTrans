
#[derive(Clone)]
struct GeoS1Interval {
    lo: f64,
    hi: f64,
}

impl GeoS1Interval {
    fn new(lo: f64, hi: f64) -> Self {
        GeoS1Interval { lo, hi }
    }

    fn is_empty(&self) -> bool {
        self.lo == std::f64::consts::PI && self.hi == -std::f64::consts::PI
    }

    fn fast_contains(&self, p: f64) -> bool {
        if self.is_inverted() {
            (p >= self.lo || p <= self.hi) && !self.is_empty()
        } else {
            p >= self.lo && p <= self.hi
        }
    }

    fn is_inverted(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        let mut l = self.hi - self.lo;
        if l < 0.0 {
            l += 2.0 * std::f64::consts::PI;
        }
        if l >= 0.0 {
            l
        } else {
            -1.0
        }
    }

    fn intersection(&self, oi: &GeoS1Interval) -> GeoS1Interval {
        if oi.is_empty() {
            return geo_s1_empty_interval();
        }
        if self.fast_contains(oi.lo) {
            if self.fast_contains(oi.hi) {
                if oi.length() < self.length() {
                    return oi.clone();
                }
                return self.clone();
            }
            return GeoS1Interval::new(oi.lo, self.hi);
        }
        if self.fast_contains(oi.hi) {
            return GeoS1Interval::new(self.lo, oi.hi);
        }
        if oi.fast_contains(self.lo) {
            return self.clone();
        }
        geo_s1_empty_interval()
    }
}

fn geo_s1_empty_interval() -> GeoS1Interval {
    GeoS1Interval::new(std::f64::consts::PI, -std::f64::consts::PI)
}
