
use std::f64::consts::PI;

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
    fn project(&self, p: f64) -> f64 {
        if p == -PI {
            return PI;
        }
        if self.fast_contains(p) {
            return p;
        }
        // Compute distance from p to each endpoint.
        let dlo = geo_s1_positive_distance(p, self.lo);
        let dhi = geo_s1_positive_distance(self.hi, p);
        if dlo < dhi {
            self.lo
        } else {
            self.hi
        }
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

    fn is_empty(&self) -> bool {
        self.lo == PI && self.hi == -PI
    }
}
