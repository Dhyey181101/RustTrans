

use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;

fn geo_s1_chord_angle_angle(c: f64) -> f64 {
    if c < 0.0 {
        -1.0 * GEO_S1_RADIAN
    } else if c.is_infinite() {
        geo_s1_inf_angle()
    } else {
        let two = 2.0;
        let half = 0.5;
        let sqrt = f64::sqrt(half * c);
        two * f64::asin(sqrt)
    }
}

fn geo_s1_chord_angle_is_infinity(c: f64) -> bool {
    c.is_infinite()
}

fn geo_s1_inf_angle() -> f64 {
    f64::INFINITY
}

type GeoS1ChordAngle = f64;
type GeoS1Angle = f64;

fn main() {
    let input: f64 = 1.0480505624698551e-13;
    if input.is_nan() {
        println!("Input is invalid, crash gracefully");
    } else {
        let angle = geo_s1_chord_angle_angle(input);
        println!("{}", angle);
    }
}

