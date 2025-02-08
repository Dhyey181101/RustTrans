

use std::f64;

fn distance(v: Box<geo_r3_Vector>, ov: Box<geo_r3_Vector>) -> f64 {
    sub(&v, &ov).norm()
}

fn sub(v: &Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> Box<geo_r3_Vector> {
    Box::new(geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    })
}

fn norm(v: &Box<geo_r3_Vector>) -> f64 {
    dot(v, v).sqrt()
}

fn dot(v: &Box<geo_r3_Vector>, ov: &Box<geo_r3_Vector>) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Clone, Copy)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_Vector {
    fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn dot(&self, ov: &Self) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

