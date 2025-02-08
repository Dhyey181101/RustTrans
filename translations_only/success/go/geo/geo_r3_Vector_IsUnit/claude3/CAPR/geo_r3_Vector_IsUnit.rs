
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

fn dot(v1: &GeoR3Vector, v2: &GeoR3Vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}
