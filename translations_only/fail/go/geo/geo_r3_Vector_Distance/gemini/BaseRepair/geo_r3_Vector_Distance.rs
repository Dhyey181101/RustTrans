
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn distance(&self, other: &Vector) -> f64 {
        sub(self, other).norm()
    }

    pub fn norm(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

fn sub(a: &Vector, b: &Vector) -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}
