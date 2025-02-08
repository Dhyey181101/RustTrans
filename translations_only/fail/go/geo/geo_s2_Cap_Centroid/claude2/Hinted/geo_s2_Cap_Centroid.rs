
use std::f64::consts::PI;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vec: GeoR3Vector,  
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

impl GeoS2Cap {
    fn centroid(&self) -> GeoS2Point {
        if self.is_empty() {
            return GeoS2Point { vec: GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 } };
        }
        let r = 1.0 - 0.5 * self.height();
        GeoS2Point {
            vec: self.center.vec.mul(r * self.area()),
        }
    }

    fn is_empty(&self) -> bool {
        self.radius < 0.0
    }

    fn height(&self) -> f64 {
        0.5 * self.radius
    }

    fn area(&self) -> f64 {
        2.0 * PI * f64::max(0.0, self.height())
    }
}

impl GeoR3Vector {
    fn mul(&self, m: f64) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}
