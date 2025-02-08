

use std::boxed::Box;

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn approx_equal(&self, other: &GeoR1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2. * GEO_R1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2. * GEO_R1_EPSILON;
        }
        let diff_lo = (other.lo - self.lo).abs();
        let diff_hi = (other.hi - self.hi).abs();
        diff_lo <= GEO_R1_EPSILON && diff_hi <= GEO_R1_EPSILON
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        self.hi - self.lo
    }
}

fn main() {}

