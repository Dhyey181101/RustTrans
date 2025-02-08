
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn normalize(v: Box<GeoR3Vector>) -> Box<GeoR3Vector> {
    let n2 = norm2(&v);
    if n2 == 0.0 {
        Box::new(GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 })
    } else {
        mul(&v, 1.0 / n2.sqrt())
    }
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(&v, &v)  
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

fn mul(v: &GeoR3Vector, m: f64) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.x * m,
        y: v.y * m, 
        z: v.z * m
    })
}
