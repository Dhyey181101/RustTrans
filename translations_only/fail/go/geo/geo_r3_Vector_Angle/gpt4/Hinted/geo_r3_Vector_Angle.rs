
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1Angle = f64;

fn angle(v: Box<GeoR3Vector>, ov: Box<GeoR3Vector>) -> GeoS1Angle {
    let cross_product = cross(&v, &ov);
    let norm = norm(&cross_product);
    let dot_product = dot(&v, &ov);
    (norm.atan2(dot_product)).abs()
}

fn cross(v: &GeoR3Vector, ov: &GeoR3Vector) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    })
}

fn norm(v: &GeoR3Vector) -> f64 {
    v.x.powi(2) + v.y.powi(2) + v.z.powi(2)
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}
