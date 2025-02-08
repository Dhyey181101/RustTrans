
use std::f64::consts::PI;

fn angle(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    (cross(v, ov).norm() / dot(v, ov)).atan() * PI
}

fn cross(v: geo_r3_Vector, ov: geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    }
}

fn norm(v: geo_r3_Vector) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

fn dot(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_Vector {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
