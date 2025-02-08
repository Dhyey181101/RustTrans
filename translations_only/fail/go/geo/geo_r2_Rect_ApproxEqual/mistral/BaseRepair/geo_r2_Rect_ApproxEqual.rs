
pub struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

pub struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

impl GeoR1Interval {
    pub fn approx_equal(&self, other: &GeoR1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2. * GEO_R1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2. * GEO_R1_EPSILON;
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

impl GeoR2Rect {
    pub fn approx_equal(&self, r2: &GeoR2Rect) -> bool {
        self.x.approx_equal(&r2.x) && self.y.approx_equal(&r2.y)
    }
}
