
use std::f64::consts::PI;

fn distance(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    (sub(v, ov)).norm()
}

fn sub(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn norm(v: &geo_r3_Vector) -> f64 {
    (dot(v, v)).sqrt()
}

fn dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Debug)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_Vector {
    fn norm(&self) -> f64 {
        (dot(self, self)).sqrt()
    }
}
