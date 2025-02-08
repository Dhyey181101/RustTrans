
use std::f64::consts::PI;

pub fn tan(c: f64) -> f64 {
    c.sin() / c.cos()
}

pub fn sin(c: f64) -> f64 {
    sin2(c).sqrt()
}

pub fn sin2(c: f64) -> f64 {
    c * (1.0 - 0.25 * c * c)
}

pub fn cos(c: f64) -> f64 {
    1.0 - 0.5 * c * c
}
