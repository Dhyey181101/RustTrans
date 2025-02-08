

use std::f64;
use std::boxed::Box;

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

pub struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    pub fn approx_equal(&self, other: &GeoR1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2.0 * GEO_R1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2.0 * GEO_R1_EPSILON;
        }
        f64::abs(other.lo - self.lo) <= GEO_R1_EPSILON && f64::abs(other.hi - self.hi) <= GEO_R1_EPSILON
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    pub fn length(&self) -> f64 {
        self.hi - self.lo
    }
}

