
use std::f64::consts::PI;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;

pub fn interior_contains_point(cap: &Cap, p: &Point) -> bool {
    is_full(cap) || chord_angle_between_points(&cap.center, p) < cap.radius
}

pub fn is_full(cap: &Cap) -> bool {
    cap.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

pub fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, (Vector::new(x.x - y.x, x.y - y.y, x.z - y.z)).norm2())
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, ov: &Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

#[derive(Copy, Clone)]
pub struct Cap {
    pub center: Point,
    pub radius: f64,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
