

use std::ops::Mul;
use std::f64::consts::SQRT_2;

pub struct GeoR3PreciseVector {
x: i128,
y: i128,
z: i128,
}

pub struct GeoR3Vector {
x: f64,
y: f64,
z: f64,
}

impl GeoR3Vector {
pub fn vector(&self) -> GeoR3Vector {
let n = self.norm2();
if n == 0.0 {
return GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 };
}
GeoR3Vector { x: self.x / n.sqrt(), y: self.y / n.sqrt(), z: self.z / n.sqrt() }
}

pub fn normalize(&self) -> GeoR3Vector {
let n2 = self.norm2();
if n2 == 0.0 {
return GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 };
}
let n = n2.sqrt();
self.mul(1.0 / n)
}

pub fn norm2(&self) -> f64 {
Self::dot(self, self)
}

pub fn dot(ov1: &GeoR3Vector, ov2: &GeoR3Vector) -> f64 {
GeoR3Vector::new(ov1.x * ov2.x, ov1.y * ov2.y, ov1.z * ov2.z).norm2()
}

pub fn mul(&self, m: f64) -> GeoR3Vector {
GeoR3Vector { x: self.x * m, y: self.y * m, z: self.z * m }
}

pub fn new(x: f64, y: f64, z: f64) -> GeoR3Vector {
GeoR3Vector { x, y, z }
}
}

fn sqrt(f: f64) -> f64 {
f.sqrt()
}

