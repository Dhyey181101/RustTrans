
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR2Rect {
    fn approx_equal(&self, r2: &GeoR2Rect) -> bool {
        self.x.approx_equal(&r2.x) && self.y.approx_equal(&r2.y)
    }
}

impl GeoR1Interval {
    fn approx_equal(&self, other: &GeoR1Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2.0 * GEO_R1_EPSILON;
        }
        if other.is_empty() {
            return self.length() <= 2.0 * GEO_R1_EPSILON;
        }
        (other.lo - self.lo).abs() <= GEO_R1_EPSILON && (other.hi - self.hi).abs() <= GEO_R1_EPSILON
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }

    fn length(&self) -> f64 {
        self.hi - self.lo
    }
}

fn main() {}
