

use std::fmt;
use std::ops;

#[derive(Copy, Clone, Default)]
struct Vector3 {
x: f64,
y: f64,
z: f64,
}

impl ops::Sub for Vector3 {
type Output = Vector3;

fn sub(self, other: Vector3) -> Vector3 {
Vector3 {
x: self.x - other.x,
y: self.y - other.y,
z: self.z - other.z,
}
}
}

impl ops::Mul<Vector3> for f64 {
type Output = Vector3;

fn mul(self, other: Vector3) -> Vector3 {
Vector3 {
x: self * other.x,
y: self * other.y,
z: self * other.z,
}
}
}

impl ops::Mul for Vector3 {
type Output = f64;

fn mul(self, other: Vector3) -> f64 {
self.x * other.x + self.y * other.y + self.z * other.z
}
}

impl ops::AddAssign for Vector3 {
fn add_assign(&mut self, other: Vector3) {
*self = Vector3 {
x: self.x + other.x,
y: self.y + other.y,
z: self.z + other.z,
}
}
}

impl Vector3 {
fn dot(&self, other: Vector3) -> f64 {
*self * other
}

fn norm2(&self) -> f64 {
self.dot(*self)
}
}

impl fmt::Debug for Vector3 {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "({}, {}, {})", self.x, self.y, self.z)
}
}

#[derive(Copy, Clone, Default)]
struct Point {
vector: Vector3,
}

impl fmt::Debug for Point {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "Point({:?})", self.vector)
}
}

#[derive(Copy, Clone, Default)]
struct Cap {
center: Point,
radius: ChordAngle,
}

impl Cap {
fn contains_point(&self, point: Point) -> bool {
let chord_angle = ChordAngle::chord_angle_between_points(self.center.vector, point.vector);
chord_angle.0 <= self.radius.0
}
}

#[derive(Copy, Clone, Default)]
struct ChordAngle(f64);

impl ChordAngle {
fn chord_angle_between_points(x: Vector3, y: Vector3) -> ChordAngle {
let dist = (x - y).norm2().min(4.0);
ChordAngle(dist.sqrt())
}
}

impl PartialOrd for ChordAngle {
fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
Some(self.0.partial_cmp(&other.0).unwrap())
}
}

impl PartialEq for ChordAngle {
fn eq(&self, other: &Self) -> bool {
self.0.eq(&other.0)
}
}

impl fmt::Debug for ChordAngle {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "ChordAngle({})", self.0)
}
}

