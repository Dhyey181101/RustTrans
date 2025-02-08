
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

fn approx_equal_r2_rect(r1: &geo_r2_Rect, r2: &geo_r2_Rect) -> bool {
    approx_equal_r1_interval(&r1.x, &r2.x) && approx_equal_r1_interval(&r1.y, &r2.y)
}

fn approx_equal_r1_interval(i1: &geo_r1_Interval, i2: &geo_r1_Interval) -> bool {
    if is_empty_r1_interval(i1) {
        return length_r1_interval(i2) <= 2.0 * GEO_R1_EPSILON;
    }
    if is_empty_r1_interval(i2) {
        return length_r1_interval(i1) <= 2.0 * GEO_R1_EPSILON;
    }
    (i2.lo - i1.lo).abs() <= GEO_R1_EPSILON && (i2.hi - i1.hi).abs() <= GEO_R1_EPSILON
}

fn is_empty_r1_interval(i: &geo_r1_Interval) -> bool {
    i.lo > i.hi
}

fn length_r1_interval(i: &geo_r1_Interval) -> f64 {
    i.hi - i.lo
}

struct geo_r2_Rect {
    x: geo_r1_Interval,
    y: geo_r1_Interval,
}

struct geo_r1_Interval {
    lo: f64,
    hi: f64,
}
