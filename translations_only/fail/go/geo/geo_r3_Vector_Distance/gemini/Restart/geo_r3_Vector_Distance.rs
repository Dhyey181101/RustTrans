
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
}

fn sub(v1: &Vector, v2: &Vector) -> Vector {
    Vector {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

impl Vector {
    pub fn norm(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
