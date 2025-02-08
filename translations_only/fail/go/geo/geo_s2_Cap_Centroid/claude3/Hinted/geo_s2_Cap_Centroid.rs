
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
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

#[derive(Debug, Clone, Copy)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Debug, Clone, Copy)]
struct GeoS1ChordAngle(f64);

#[derive(Debug, Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
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

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    fn height(&self) -> f64 {
        0.5 * self.radius.0
    }

    fn area(&self) -> f64 {
        2.0 * PI * self.height().max(0.0)
    }
}
