
use std::f64::consts::PI;

pub type GeoR3Vector = (f64, f64, f64);
pub type GeoS1Angle = f64;

pub fn angle(v: GeoR3Vector, ov: GeoR3Vector) -> GeoS1Angle {
    let (x, y, z) = v;
    let (ox, oy, oz) = ov;
    (PI / 2.0) * (y * oz - z * oy).atan2(x * ox + y * oy + z * oz)
}

pub fn cross(v: GeoR3Vector, ov: GeoR3Vector) -> GeoR3Vector {
    let (x, y, z) = v;
    let (ox, oy, oz) = ov;
    (y * oz - z * oy, z * ox - x * oz, x * oy - y * ox)
}

pub fn norm(v: GeoR3Vector) -> f64 {
    let (x, y, z) = v;
    (x * x + y * y + z * z).sqrt()
}

pub fn dot(v: GeoR3Vector, ov: GeoR3Vector) -> f64 {
    let (x, y, z) = v;
    let (ox, oy, oz) = ov;
    x * ox + y * oy + z * oz
}
