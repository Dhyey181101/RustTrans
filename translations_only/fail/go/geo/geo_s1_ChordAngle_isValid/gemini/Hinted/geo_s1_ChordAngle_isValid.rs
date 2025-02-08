
use std::f64::consts::PI;
use std::f64::EPSILON;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

fn is_valid(c: f64) -> bool {
    (c >= 0.0 && c <= GEO_S1_MAX_LENGTH2) || is_special(c)
}

fn is_special(c: f64) -> bool {
    c < 0.0 || c.is_infinite() || c.is_nan()
}

fn is_infinite(c: f64) -> bool {
    c.abs() > EPSILON * PI
}
