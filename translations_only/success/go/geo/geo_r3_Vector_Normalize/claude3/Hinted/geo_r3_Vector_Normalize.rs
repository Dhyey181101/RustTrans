
use std::ops;

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn normalize(self) -> GeoR3Vector {
        let n2 = self.norm2();
        if n2 == 0.0 {
            GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 }
        } else {
            mul_scalar(self, 1.0 / n2.sqrt())
        }
    }

    fn norm2(self) -> f64 {
        dot(self, self)
    }
}

fn dot(v: GeoR3Vector, ov: GeoR3Vector) -> f64 {
    (v.x * ov.x) + (v.y * ov.y) + (v.z * ov.z)
}

fn mul_scalar(v: GeoR3Vector, m: f64) -> GeoR3Vector {
    GeoR3Vector {
        x: m * v.x,
        y: m * v.y,
        z: m * v.z,
    }
}
