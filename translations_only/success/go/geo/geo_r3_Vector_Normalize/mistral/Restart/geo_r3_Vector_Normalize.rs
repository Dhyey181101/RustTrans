

use std::f64;

#[derive(Clone)]
struct GeoR3Vector {
x: f64,
y: f64,
z: f64,
}

impl GeoR3Vector {
fn normalize(&self) -> GeoR3Vector {
let n2 = self.norm2();
if n2 == 0.0 {
return GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 };
}
let v = self.clone();
v.mul(1.0 / n2.sqrt())
}

fn norm2(&self) -> f64 {
self.dot(&self)
}

fn dot(&self, ov: &GeoR3Vector) -> f64 {
self.x * ov.x + self.y * ov.y + self.z * ov.z
}

fn mul(&self, m: f64) -> GeoR3Vector {
GeoR3Vector { x: m * self.x, y: m * self.y, z: m * self.z }
}
}

fn norm2(v: &GeoR3Vector) -> f64 {
v.dot(v)
}

