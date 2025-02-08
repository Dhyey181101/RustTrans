
use std::f64::consts::PI;

fn tan(c: f64) -> f64 {
    sin(c) / cos(c)
}

fn sin(c: f64) -> f64 {
    (sin2(c)).sqrt()
}

fn sin2(c: f64) -> f64 {
    // Let a be the (non-squared) chord length, and let A be the corresponding
    // half-angle (a = 2*sin(A)). The formula below can be derived from:
    //   sin(2*A) = 2 * sin(A) * cos(A)
    //   cos^2(A) = 1 - sin^2(A)
    // This is much faster than converting to an angle and computing its sine.
    c * (1.0 - 0.25 * c)
}

fn cos(c: f64) -> f64 {
    // cos(2*A) = cos^2(A) - sin^2(A) = 1 - 2*sin^2(A)
    1.0 - 0.5 * c
}
