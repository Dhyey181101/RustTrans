
const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

fn approx_equal_r2_rect(r1: &Rect, r2: &Rect) -> bool {
    approx_equal_r1_interval(&r1.x, &r2.x) && approx_equal_r1_interval(&r1.y, &r2.y)
}

fn approx_equal_r1_interval(i1: &Interval, i2: &Interval) -> bool {
    if is_empty_r1_interval(i1) {
        length_r1_interval(i2) <= 2.0 * GEO_R1_EPSILON
    } else if is_empty_r1_interval(i2) {
        length_r1_interval(i1) <= 2.0 * GEO_R1_EPSILON
    } else {
        (i2.lo - i1.lo).abs() <= GEO_R1_EPSILON && (i2.hi - i1.hi).abs() <= GEO_R1_EPSILON
    }
}

fn is_empty_r1_interval(i: &Interval) -> bool {
    i.lo > i.hi
}

fn length_r1_interval(i: &Interval) -> f64 {
    i.hi - i.lo
}

struct Rect {
    x: Interval,
    y: Interval,
}

struct Interval {
    lo: f64,
    hi: f64,
}
