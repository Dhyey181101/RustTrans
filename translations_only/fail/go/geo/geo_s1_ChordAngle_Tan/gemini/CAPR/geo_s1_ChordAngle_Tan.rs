
use std::f64::consts::PI;

pub fn tan(c: f64) -> f64 {
    sin(c) / cos(c)
}

pub fn sin(c: f64) -> f64 {
    c.sqrt()
}

pub fn sin2(c: f64) -> f64 {
    c * (1.0 - 0.25 * c)
}

pub fn cos(c: f64) -> f64 {
    1.0 - 0.5 * c
}
