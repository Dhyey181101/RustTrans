
use std::f64::consts::PI;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: Option<f64>, // Using Option<f64> to represent geo_s1_ChordAngle which can be None
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoS2Cap {
    fn is_empty(&self) -> bool {
        match self.radius {
            Some(radius) => radius < 0.0,
            None => true,
        }
    }

    fn height(&self) -> f64 {
        match self.radius {
            Some(radius) => 0.5 * radius,
            None => 0.0,
        }
    }

    fn area(&self) -> f64 {
        2.0 * PI * self.height().max(0.0)
    }
}

fn centroid(cap: &GeoS2Cap) -> GeoS2Point {
    if cap.is_empty() {
        return GeoS2Point {
            vector: GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 },
        };
    }
    let r = 1.0 - 0.5 * cap.height();
    GeoS2Point {
        vector: mul(&cap.center.vector, r * cap.area()),
    }
}

fn mul(v: &GeoR3Vector, m: f64) -> GeoR3Vector {
    GeoR3Vector {
        x: m * v.x,
        y: m * v.y,
        z: m * v.z,
    }
}
