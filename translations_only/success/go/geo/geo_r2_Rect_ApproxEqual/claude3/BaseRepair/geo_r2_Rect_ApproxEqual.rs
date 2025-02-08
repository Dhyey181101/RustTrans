
use std::f64;

const GEO_S2_EPSILON: f64 = 1e-15;
const GEO_R1_EPSILON: f64 = 1e-15;

fn geo_r2_rect_approx_equal(r: Box<geo_r2_Rect>, r2: Box<geo_r2_Rect>) -> bool {
    geo_r1_interval_approx_equal(r.X, r2.X) && geo_r1_interval_approx_equal(r.Y, r2.Y)
}

fn geo_r1_interval_approx_equal(i: Box<geo_r1_Interval>, other: Box<geo_r1_Interval>) -> bool {
    if geo_r1_interval_is_empty(&i) {
        return geo_r1_interval_length(&other) <= 2.0 * GEO_R1_EPSILON;
    }
    if geo_r1_interval_is_empty(&other) {
        return geo_r1_interval_length(&i) <= 2.0 * GEO_R1_EPSILON;
    }
    (other.Lo - i.Lo).abs() <= GEO_R1_EPSILON && (other.Hi - i.Hi).abs() <= GEO_R1_EPSILON
}

fn geo_r1_interval_is_empty(i: &geo_r1_Interval) -> bool {
    i.Lo > i.Hi
}

fn geo_r1_interval_length(i: &geo_r1_Interval) -> f64 {
    i.Hi - i.Lo
}

#[derive(Clone)]
struct geo_r2_Rect {
    X: Box<geo_r1_Interval>,
    Y: Box<geo_r1_Interval>,
}

#[derive(Clone)]
struct geo_r1_Interval {
    Lo: f64,
    Hi: f64,
}
