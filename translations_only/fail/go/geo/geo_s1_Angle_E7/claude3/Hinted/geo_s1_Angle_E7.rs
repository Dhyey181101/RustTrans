

use std::f64::consts::PI;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (PI / 180.0) * GEO_S1_RADIAN;

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    return (val + 0.5) as i32;
}

fn geo_s1_angle_e7(a: f64) -> i32 {
    geo_s1_round(a.to_degrees() * 1e7)
}

fn geo_s1_angle_degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

fn is_finite(val: f64) -> bool {
    val.is_finite()
}

fn main() {
    let test_cases = [
        5.641404760978848e-256,
        5.339777096725099e-256,
    ];

    for case in &test_cases {
        if is_finite(*case) {
            let e7 = geo_s1_angle_e7(*case);
            let degrees = geo_s1_angle_degrees(*case);
            println!("e7: {}, degrees: {}", e7, degrees);
        } else {
            println!("Input is invalid, crash gracefully");
        }
    }
}

