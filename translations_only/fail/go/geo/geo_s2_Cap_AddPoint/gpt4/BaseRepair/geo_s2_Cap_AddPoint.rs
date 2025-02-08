
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
    vector: Box<GeoR3Vector>,
}

struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: Box<GeoS2Point>,
    radius: GeoS1ChordAngle,
}

impl GeoS2Cap {
    fn add_point(&mut self, p: Box<GeoS2Point>) -> Self {
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
}

fn geo_s2_chord_angle_between_points(x: &Box<GeoS2Point>, y: &Box<GeoS2Point>) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, x.vector.sub(&y.vector).norm2()))
}

impl Clone for GeoS2Cap {
    fn clone(&self) -> Self {
        GeoS2Cap {
            center: Box::new((*self.center).clone()),
            radius: GeoS1ChordAngle(self.radius.0),
        }
    }
}

impl Clone for GeoS2Point {
    fn clone(&self) -> Self {
        GeoS2Point {
            vector: Box::new(*self.vector.clone()),
        }
    }
}

impl Clone for GeoR3Vector {
    fn clone(&self) -> Self {
        GeoR3Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
