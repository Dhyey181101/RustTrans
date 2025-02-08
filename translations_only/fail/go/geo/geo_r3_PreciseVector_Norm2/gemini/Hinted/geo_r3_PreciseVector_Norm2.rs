
use std::ops::{Add, Mul};

const GEO_R3_PREC: usize = 1024;

fn norm2(v: &geo_r3_PreciseVector) -> Box<f64> {
    dot(v, v)
}

fn dot(v: &geo_r3_PreciseVector, ov: &geo_r3_PreciseVector) -> Box<f64> {
    Box::new(v.x * ov.x + v.y * ov.y + v.z * ov.z)
}

fn geo_r3_prec_mul(a: &f64, b: &f64) -> Box<f64> {
    Box::new(a * b)
}

fn geo_r3_prec_add(a: &f64, b: &f64) -> Box<f64> {
    Box::new(a + b)
}

#[derive(Debug)]
struct geo_r3_PreciseVector {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for geo_r3_PreciseVector {
    type Output = geo_r3_PreciseVector;

    fn add(self, other: geo_r3_PreciseVector) -> geo_r3_PreciseVector {
        geo_r3_PreciseVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for geo_r3_PreciseVector {
    type Output = geo_r3_PreciseVector;

    fn mul(self, other: geo_r3_PreciseVector) -> geo_r3_PreciseVector {
        geo_r3_PreciseVector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
