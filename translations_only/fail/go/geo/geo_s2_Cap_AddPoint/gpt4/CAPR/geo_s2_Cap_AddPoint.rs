
use std::f64::consts::PI;

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, other: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, other: &GeoR3Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
}

struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn add_point(&mut self, p: Box<GeoS2Point>) -> GeoS2Cap {
        if self.is_empty() {
            self.center = p;
            self.radius = GeoS1ChordAngle(0.0);
            return self.clone();
        }

        let new_rad = geo_s2_chord_angle_between_points(&self.center, &p);
        if new_rad.0 > self.radius.0 {
            self.radius = new_rad;
        }
        self.clone()
    }

    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    fn clone(&self) -> GeoS2Cap {
        GeoS2Cap {
            center: Box::new(GeoS2Point {
                geo_r3_vector: Box::new(GeoR3Vector {
                    x: self.center.geo_r3_vector.x,
                    y: self.center.geo_r3_vector.y,
                    z: self.center.geo_r3_vector.z,
                }),
            }),
            radius: GeoS1ChordAngle(self.radius.0),
        }
    }
}

fn geo_s2_chord_angle_between_points(x: &Box<GeoS2Point>, y: &Box<GeoS2Point>) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(
        4.0,
        x.geo_r3_vector.sub(&y.geo_r3_vector).norm2(),
    ))
}

fn main() {}
