
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Cap {
    center: Point,
    radius: ChordAngle,
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

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn sub(&self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ChordAngle(pub f64);

pub fn chord_angle_between_points(x: Point, y: Point) -> ChordAngle {
    ChordAngle(f64::min(4.0, x.sub(&y).norm2().sqrt()))
}
