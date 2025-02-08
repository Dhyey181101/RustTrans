

use std::f64;
use std::isize;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_inf_angle() -> f64 {
f64::INFINITY
}

fn geo_s1_angle(c: f64, _r3_vector: &GeoR3Vector) -> f64 {
if c < 0.0 {
-1.0 * GEO_S1_RADIAN
} else if c.is_infinite() {
geo_s1_inf_angle()
} else {
2.0 * f64::asin(0.5 * f64::sqrt(c * 0.5))
}
}

fn geo_s1_is_infinity(c: f64) -> bool {
c.is_infinite()
}

#[derive(Copy, Clone, Debug)]
struct GeoS2Cap {
radius: GeoS1ChordAngle,
}

#[derive(Copy, Clone, Debug)]
struct GeoS2Point {
geo_r3_vector: GeoR3Vector,
}

#[derive(Copy, Clone, Debug)]
struct GeoR3Vector {
}

#[derive(Copy, Clone, Debug)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone, Debug)]
struct GeoS1Angle(f64);

