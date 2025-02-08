
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

fn approx_equal_r2(r: &GeoR2Rect, r2: &GeoR2Rect) -> bool {
    approx_equal_r1(&r.x, &r2.x) && approx_equal_r1(&r.y, &r2.y)
}

fn approx_equal_r1(i: &GeoR1Interval, other: &GeoR1Interval) -> bool {
    if is_empty_r1(i) {
        return length_r1(other) <= 2.0 * GEO_R1_EPSILON;
    }
    if is_empty_r1(other) {
        return length_r1(i) <= 2.0 * GEO_R1_EPSILON;
    }
    (other.lo - i.lo).abs() <= GEO_R1_EPSILON && (other.hi - i.hi).abs() <= GEO_R1_EPSILON
}

fn is_empty_r1(i: &GeoR1Interval) -> bool {
    i.lo > i.hi
}

fn length_r1(i: &GeoR1Interval) -> f64 {
    i.hi - i.lo
}
