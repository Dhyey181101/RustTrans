
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

#[derive(Debug, Clone, Copy)]
pub struct GeoR1Interval {
    pub lo: f64,
    pub hi: f64,
}

impl GeoR1Interval {
    pub fn approx_equal(&self, other: GeoR1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2.0 * GEO_R1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2.0 * GEO_R1_EPSILON;
        }
        (other.lo - self.lo).abs() <= GEO_R1_EPSILON && (other.hi - self.hi).abs() <= GEO_R1_EPSILON
    }

    pub fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    pub fn length(&self) -> f64 {
        self.hi - self.lo
    }
}
