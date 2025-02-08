

use std::f64;

#[derive(Clone)]
struct GeoR3Vector {
x: f64,
y: f64,
z: f64,
}

impl GeoR3Vector {
fn norm2(&self) -> f64 {
self.dot(self.clone())
}

fn dot(&self, ov: GeoR3Vector) -> f64 {
self.x * ov.x + self.y * ov.y + self.z * ov.z
}

fn mul(&self, m: f64) -> GeoR3Vector {
GeoR3Vector {
x: m * self.x,
y: m * self.y,
z: m * self.z,
}
}
}

fn normalize(v: &GeoR3Vector) -> GeoR3Vector {
let n2 = v.norm2();
if n2 == 0.0 {
GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 }
} else {
v.mul(1.0 / f64::sqrt(n2))
}
}

