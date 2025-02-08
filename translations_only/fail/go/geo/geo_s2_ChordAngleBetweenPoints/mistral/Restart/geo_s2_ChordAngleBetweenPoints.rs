

use std::f64;
use crate::geo::Point;
use crate::geo::Vector;
use crate::geo::ChordAngle; // added

mod geo {
pub(crate) struct Vector {
x: f64,
y: f64,
z: f64,
}

pub struct ChordAngle(pub f64);

pub struct Point(Vector);

impl Point {
pub fn sub(&self, other: &Point) -> Vector {
Vector {
x: self.0.x - other.0.x,
y: self.0.y - other.0.y,
z: self.0.z - other.0.z,
}
}
}

impl Vector {
pub fn dot(&self, other: &Vector) -> f64 {
self.x * other.x + self.y * other.y + self.z * other.z
}

pub fn norm2(&self) -> f64 {
self.dot(self)
}
}
}

pub fn geo_s2_chord_angle_between_points(x: Point, y: Point) -> Box<ChordAngle> {
let diff = x.sub(&y);
let dist = diff.norm2().min(4.0);
Box::new(ChordAngle(dist))
}

