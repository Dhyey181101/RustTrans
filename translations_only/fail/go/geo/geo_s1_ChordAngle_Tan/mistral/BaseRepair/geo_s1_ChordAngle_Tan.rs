

use std::f64;

fn tan(c: f64) -> f64 {
c / cos(c)
}

fn sin(c: f64) -> f64 {
f64::sqrt(sin2(c))
}

fn sin2(c: f64) -> f64 {
c * (1.0 - (0.25 * c))
}

fn cos(c: f64) -> f64 {
1.0 - (0.5 * c)
}

