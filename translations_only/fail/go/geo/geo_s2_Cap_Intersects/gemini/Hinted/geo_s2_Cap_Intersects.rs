
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

#[derive(Debug, Clone, Copy)]
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }
}

#[derive(Debug, Clone, Copy)]
struct GeoS1ChordAngle(f64);

#[derive(Debug, Clone, Copy)]
struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn intersects(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }

        self.radius.0 + other.radius.0 >= geo_s2_chord_angle_between_points(&self.center, &other.center)
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> f64 {
    f64::min(4.0, x.geo_r3_vector.sub(&y.geo_r3_vector).norm2().sqrt())
}

impl GeoS1ChordAngle {
    fn add(&self, other: &Self) -> Self {
        let c = self.0 + other.0;
        if c >= 4.0 {
            return GeoS1ChordAngle(4.0);
        }

        let x = c * (1.0 - 0.25 * other.0);
        let y = other.0 * (1.0 - 0.25 * c);
        GeoS1ChordAngle(f64::min(4.0, x + y + 2.0 * (x * y).sqrt()))
    }
}

fn main() {
    let cap1 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_vector: GeoR3Vector { x: 0.0, y: 0.0, z: 0.0 },
        },
        radius: GeoS1ChordAngle(0.0),
    };
    let cap2 = GeoS2Cap {
        center: GeoS2Point {
            geo_r3_vector: GeoR3Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        radius: GeoS1ChordAngle(PI),
    };
    println!("{}", cap1.intersects(&cap2));
}
