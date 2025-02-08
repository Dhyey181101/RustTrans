

use std::boxed::Box;
use std::str::FromStr;

const GEO_R3_PREC: i32 = 128;

struct GeoR3PreciseVector {
    x: Box<i128>,
    y: Box<i128>,
    z: Box<i128>,
}

fn mul_by_float64(v: GeoR3PreciseVector, f: f64) -> GeoR3PreciseVector {
    let f = (f * 10_f64.powf(GEO_R3_PREC as f64 / 3.0)) as i128;
    GeoR3PreciseVector {
        x: Box::new(*v.x * f),
        y: Box::new(*v.y * f),
        z: Box::new(*v.z * f),
    }
}

fn geo_r3_prec_float(f: f64) -> Box<i128> {
    let mut f_big = (10_f64.powf(GEO_R3_PREC as f64 % 3.0) * (10 as f64).powf(GEO_R3_PREC as f64 / 3.0 * 3.0)) as i128;
    Box::new(f_big)
}

fn mul(v: GeoR3PreciseVector, f: &i128) -> GeoR3PreciseVector {
    GeoR3PreciseVector {
        x: Box::new(*v.x * f),
        y: Box::new(*v.y * f),
        z: Box::new(*v.z * f),
    }
}

fn geo_r3_prec_mul(a: &i128, b: &i128) -> Box<i128> {
    let mut result = String::new();
    let a_big = a.to_string();
    let b_big = b.to_string();
    for a_char in a_big.chars() {
        for b_char in b_big.chars() {
            let a_val = i32::from_str_radix(&a_char.to_string(), 10).unwrap();
            let b_val = i32::from_str_radix(&b_char.to_string(), 10).unwrap();
            let prod = (a_val * b_val) % 10;
            result.push_str(&prod.to_string());
        }
    }
    result.drain(0..result.len() - (GEO_R3_PREC as usize) / 3);
    Box::new(i128::from_str_radix(&result, 10).unwrap())
}

