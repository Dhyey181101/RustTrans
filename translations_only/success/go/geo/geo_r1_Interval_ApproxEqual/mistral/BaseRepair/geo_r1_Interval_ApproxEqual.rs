
use std::boxed::Box;

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

#[derive(Debug, Clone, Copy)]
struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        self.hi - self.lo
    }
}

fn approx_equal(i: &GeoR1Interval, other: &GeoR1Interval) -> bool {
    if i.is_empty() {
        return other.length() <= 2.0 * GEO_R1_EPSILON;
    }
    if other.is_empty() {
        return i.length() <= 2.0 * GEO_R1_EPSILON;
    }
    (other.lo - i.lo).abs() <= GEO_R1_EPSILON && (other.hi - i.hi).abs() <= GEO_R1_EPSILON
}

