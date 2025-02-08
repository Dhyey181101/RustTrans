
use std::f64::consts::SQRT_2;

#[derive(Clone)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn distance(v: Box<GeoR3Vector>, ov: Box<GeoR3Vector>) -> f64 {
    norm(&*sub(v, ov))  
}

fn sub(v: Box<GeoR3Vector>, ov: Box<GeoR3Vector>) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    })
}

fn norm(v: &GeoR3Vector) -> f64 {
    f64::sqrt(dot(v, v))
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z  
}

