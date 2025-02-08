
use std::f64::consts::PI;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

impl GeoS2Cap {
    fn is_empty(&self) -> bool {
        self.radius < 0.0
    }

    fn height(&self) -> f64 {
        0.5 * self.radius
    }

    fn area(&self) -> f64 {
        2.0 * PI * self.height().max(0.0)
    }

    fn centroid(&self) -> GeoS2Point {
        if self.is_empty() {
            return GeoS2Point {
                geo_r3_vector: GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 },
            };
        }
        let r = 1.0 - 0.5 * self.height();
        GeoS2Point {
            geo_r3_vector: self.center.geo_r3_vector.mul(r * self.area()),
        }
    }
}

impl GeoR3Vector {
    fn mul(&self, m: f64) -> GeoR3Vector {
        GeoR3Vector {
            x: m * self.x,
            y: m * self.y,
            z: m * self.z,
        }
    }
}
