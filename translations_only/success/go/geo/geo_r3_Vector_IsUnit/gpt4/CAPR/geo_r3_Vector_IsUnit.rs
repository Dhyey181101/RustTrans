
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

fn is_unit(v: &GeoR3Vector) -> bool {
    const EPSILON: f64 = 5e-14;
    (norm2(v) - 1.0).abs() <= EPSILON
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}
