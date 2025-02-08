
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (f64::consts::PI / 180.0) * GEO_S1_RADIAN;

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        return (val - 0.5) as i32;
    }
    return (val + 0.5) as i32;
}

fn geo_s1_e6(a: f64) -> i32 {
    geo_s1_round(a * 1e6)
}

fn geo_s1_degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

fn main() {
    let test_cases = vec![
        -9.302382603986702e-105 as f64,
        9.529057115e-315 as f64,
        1.0316768614569301e+26 as f64,
        1.1962166766497118e+103 as f64,
    ];

    for tc in test_cases {
        println!("{}: {}", tc, geo_s1_e6(geo_s1_degrees(tc)));
    }
}
