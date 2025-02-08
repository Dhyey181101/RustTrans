
use std::f64;

const GEO_S1_RADIAN: f64 = 1.0;
const GEO_S1_DEGREE: f64 = (f64::consts::PI / 180.0) * GEO_S1_RADIAN;

fn e5(a: f64) -> i32 {
    geo_s1_round(a * 1e5)
}

fn degrees(a: f64) -> f64 {
    a / GEO_S1_DEGREE
}

fn geo_s1_round(val: f64) -> i32 {
    if val < 0.0 {
        val as i32 - 1 - (val as i32 % 1)
    } else {
        val as i32 + (val as i32 % 1)
    }
}

fn main() {
    let input: [f64; 4] = [
        6.021606341785306e-307,
        6.359078365286997e-250,
        1.0567780951338947e+305,
        7.746749634277764e+120,
    ];
    for i in input {
        println!("{}", e5(degrees(i)));
    }
}
