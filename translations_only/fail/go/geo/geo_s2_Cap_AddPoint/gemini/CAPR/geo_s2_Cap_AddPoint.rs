
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Cap {
    pub center: Point,
    pub radius: ChordAngle,
}

impl Cap {
    pub fn add_point(&mut self, p: Point) {
        if self.is_empty() {
            self.center = p;
            self.radius = ChordAngle(0.0);
            return;
        }

        let new_rad = chord_angle_between_points(self.center, p);
        if new_rad > self.radius {
            self.radius = new_rad;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ChordAngle(pub f64);

pub fn chord_angle_between_points(x: Point, y: Point) -> ChordAngle {
    ChordAngle(f64::min(4.0, x.sub(&y).norm2().sqrt()))
}
