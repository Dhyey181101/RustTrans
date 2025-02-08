

use std::f64;

type geo_r3_Vector = Vector;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn sub(&self, ov: Vector) -> Vector {
        Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn dot(&self, ov: &Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }

    fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

fn distance(v: Vector, ov: Vector) -> f64 {
    let sub = v.sub(ov);
    sub.norm()
}

