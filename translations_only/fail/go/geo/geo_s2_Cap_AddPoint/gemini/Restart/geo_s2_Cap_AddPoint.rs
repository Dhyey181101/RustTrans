

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Cap {
    center: Point,
    radius: ChordAngle,
}

impl Cap {
    pub fn new(center: Point, radius: ChordAngle) -> Self {
        Self { center, radius }
    }

    pub fn add_point(&mut self, p: Point) {
        if self.is_empty() {
            self.center = p;
            self.radius = ChordAngle(0.0);
            return;
        }

        let new_rad = chord_angle_between_points(self.center, p);
        if new_rad.0 > self.radius.0 {
            self.radius = new_rad;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn sub(&self, other: &Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ChordAngle(pub f64);

impl ChordAngle {
    pub fn new(rad: f64) -> Self {
        Self(rad)
    }
}

pub fn chord_angle_between_points(x: Point, y: Point) -> ChordAngle {
    ChordAngle(f64::min(4.0, x.sub(&y).norm2().sqrt()))
}

