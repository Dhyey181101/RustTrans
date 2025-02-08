
const EPSILON: f64 = 5e-14;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn is_unit(&self) -> bool {
        (self.norm2() - 1.0).abs() <= EPSILON
    }

    fn norm2(&self) -> f64 {
        dot(self, self)
    }
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}
