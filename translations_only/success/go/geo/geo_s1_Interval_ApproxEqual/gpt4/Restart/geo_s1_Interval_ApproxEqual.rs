
const GEO_S1_EPSILON: f64 = 1e-15;

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

    fn length(&self) -> f64 {
        let mut l = self.hi - self.lo;
        if l >= 0.0 {
            return l;
        }
        l += 2.0 * std::f64::consts::PI;
        if l > 0.0 {
            return l;
        }
        -1.0
    }

    fn is_full(&self) -> bool {
        self.lo == -std::f64::consts::PI && self.hi == std::f64::consts::PI
    }
}

fn approx_equal(i: &GeoS1Interval, other: &GeoS1Interval) -> bool {
    if i.is_empty() {
        return other.length() <= 2.0 * GEO_S1_EPSILON;
    }
    if other.is_empty() {
        return i.length() <= 2.0 * GEO_S1_EPSILON;
    }
    if i.is_full() {
        return other.length() >= 2.0 * (std::f64::consts::PI - GEO_S1_EPSILON);
    }
    if other.is_full() {
        return i.length() >= 2.0 * (std::f64::consts::PI - GEO_S1_EPSILON);
    }

    (f64::abs(((other.lo - i.lo) % (2.0 * std::f64::consts::PI))) <= GEO_S1_EPSILON &&
     f64::abs(((other.hi - i.hi) % (2.0 * std::f64::consts::PI))) <= GEO_S1_EPSILON &&
     f64::abs(i.length() - other.length()) <= 2.0 * GEO_S1_EPSILON)
}
