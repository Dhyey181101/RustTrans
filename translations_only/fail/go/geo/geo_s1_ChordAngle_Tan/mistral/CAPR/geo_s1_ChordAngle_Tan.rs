

use std::f64;
use std::ops::Mul;

fn tan(c: f64) -> f64 {
    c.sin() / c.cos()
}

fn sin(c: f64) -> f64 {
    f64::sqrt(sin2(c))
}

fn sin2(c: f64) -> f64 {
    c * (1.0 - 0.25 * c)
}

fn cos(c: f64) -> f64 {
    1.0 - 0.5 * c
}

