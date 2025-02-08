
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

pub fn approx_equal_r2_rect(r1: &GeoR2Rect, r2: &GeoR2Rect) -> bool {
    approx_equal_r1_interval(&r1.x, &r2.x) && approx_equal_r1_interval(&r1.y, &r2.y)
}

pub fn approx_equal_r1_interval(i1: &GeoR1Interval, i2: &GeoR1Interval) -> bool {
    if is_empty_r1_interval(i1) {
        length_r1_interval(i2) <= 2.0 * GEO_R1_EPSILON
    } else if is_empty_r1_interval(i2) {
        length_r1_interval(i1) <= 2.0 * GEO_R1_EPSILON
    } else {
        (i2.lo - i1.lo).abs() <= GEO_R1_EPSILON && (i2.hi - i1.hi).abs() <= GEO_R1_EPSILON
    }
}

pub fn is_empty_r1_interval(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

pub fn length_r1_interval(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}

#[derive(Debug, Clone, Copy)]
pub struct GeoR2Rect {
    pub x: GeoR1Interval,
    pub y: GeoR1Interval,
}

#[derive(Debug, Clone, Copy)]
pub struct GeoR1Interval {
    pub lo: f64,
    pub hi: f64,
}
