

const EPSILON: f64 = 5e-14;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn is_unit(v: &GeoR3Vector) -> bool {
    (norm2(&v) - 1.0).abs() <= EPSILON
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(v, v)
}

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

