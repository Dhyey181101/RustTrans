

use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_angle(c: f64) -> f64 {
if c < 0.0 {
-GEO_S1_RADIAN
} else if c > 0.0 {
let c_sqrt = c.sqrt();
f64::consts::PI - (0.5 * c_sqrt / (1.0 + c_sqrt)).atan()
} else {
f64::consts::PI
}
}

fn geo_s1_is_infinity(c: f64) -> bool {
c.is_infinite()
}

fn main() {
let _ = geo_s1_angle(8.955920566347428e-135);
let _ = geo_s1_angle(-4.7684799933493603e-05);
let _ = geo_s1_is_infinity(f64::INFINITY);
let _ = geo_s1_is_infinity(-f64::INFINITY);
}

