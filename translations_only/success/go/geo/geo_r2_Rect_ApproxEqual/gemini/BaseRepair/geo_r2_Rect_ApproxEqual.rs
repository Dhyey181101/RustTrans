
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

fn approx_equal_r2_rect(r1: &GeoR2Rect, r2: &GeoR2Rect) -> bool {
    approx_equal_r1_interval(&r1.x, &r2.x) && approx_equal_r1_interval(&r1.y, &r2.y)
}

fn approx_equal_r1_interval(i1: &GeoR1Interval, i2: &GeoR1Interval) -> bool {
    if i1.is_empty() {
        i2.length() <= 2.0 * GEO_R1_EPSILON
    } else if i2.is_empty() {
        i1.length() <= 2.0 * GEO_R1_EPSILON
    } else {
        (i2.lo - i1.lo).abs() <= GEO_R1_EPSILON && (i2.hi - i1.hi).abs() <= GEO_R1_EPSILON
    }
}

fn is_empty_r1_interval(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn length_r1_interval(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}

struct GeoR2Rect {
    x: GeoR1Interval,
    y: GeoR1Interval,
}

struct GeoR1Interval {
    lo: f64,
    hi: f64,
}

impl GeoR1Interval {
    fn length(&self) -> f64 {
        self.hi - self.lo
    }

    fn approx_equal(&self, other: &Self) -> bool {
        if self.is_empty() {
            other.length() <= 2.0 * GEO_R1_EPSILON
        } else if other.is_empty() {
            self.length() <= 2.0 * GEO_R1_EPSILON
        } else {
            (other.lo - self.lo).abs() <= GEO_R1_EPSILON && (other.hi - self.hi).abs() <= GEO_R1_EPSILON
        }
    }

    fn is_empty(&self) -> bool {
        self.lo > self.hi
    }
}
