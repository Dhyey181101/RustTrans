
use std::f64::consts::PI;
use std::f64::INFINITY;

const GEO_S1_MAX_LENGTH2: f64 = 4.0;

pub fn is_valid(c: f64) -> bool {
    (c >= 0.0 && c <= GEO_S1_MAX_LENGTH2) || is_special(c)
}

pub fn is_special(c: f64) -> bool {
    c < 0.0 || c.is_infinite()
}

pub fn is_infinity(c: f64) -> bool {
    c.is_infinite()
}
