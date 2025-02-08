
use std::f64::consts::PI;

fn tan(c: f64) -> f64 {
    sin(c) / cos(c)
}

fn sin(c: f64) -> f64 {
    c.abs().sqrt() * if c < 0.0 {-1.0} else {1.0}
}

fn sin2(c: f64) -> f64 {
    if c.is_nan() || c.abs() > 1.0 {
        panic!("Invalid input"); 
    }
    c * (1.0 - 0.25 * c)
}

fn cos(c: f64) -> f64 {
    if c.is_nan() || c.abs() > 1.0 {
        panic!("Invalid input");
    }
    1.0 - 0.5 * c
}

