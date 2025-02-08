
use std::f64::consts::SQRT_2;

pub fn normalize(v: geo_r3_Vector) -> geo_r3_Vector {
    let n2 = norm2(v);
    if n2 == 0.0 {
        return geo_r3_Vector { x: 0.0, y: 0.0, z: 0.0 };
    }
    mul(v, 1.0 / SQRT_2 * n2)
}

pub fn norm2(v: geo_r3_Vector) -> f64 {
    dot(v, v)
}

pub fn dot(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

pub fn mul(v: geo_r3_Vector, m: f64) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x * m,
        y: v.y * m,
        z: v.z * m,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct geo_r3_Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
