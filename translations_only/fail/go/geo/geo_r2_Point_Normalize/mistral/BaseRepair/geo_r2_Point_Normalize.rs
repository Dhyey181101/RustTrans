

use std::f64;
use std::boxed::Box;

struct GeoR2Point {
x: f64,
y: f64,
}

impl GeoR2Point {
fn norm(&self) -> f64 {
f64::hypot(self.x, self.y)
}
}

fn normalize(mut p: GeoR2Point) -> GeoR2Point {
if p.x == 0.0 && p.y == 0.0 {
return p;
}
let norm = p.norm();
GeoR2Point {
x: p.x / norm,
y: p.y / norm,
}
}

fn mul(m: f64, p: &GeoR2Point) -> GeoR2Point {
GeoR2Point {
x: m * p.x,
y: m * p.y,
}
}

