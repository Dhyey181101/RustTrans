

use std::fmt;
use std::ops::Add;

const GEO_S1_STRAIGHT_CHORD_ANGLE: f64 = 4.0;
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

#[derive(Copy, Clone, PartialEq, Debug)]
struct GeoS1ChordAngle(f64);

impl GeoS1ChordAngle {
fn add(self, other: GeoS1ChordAngle) -> GeoS1ChordAngle {
if other.0 == 0.0 {
return self;
}

let c = self.0;
let other = other.0;

if c + other >= GEO_S1_MAX_LENGTH2 {
return GeoS1ChordAngle(GEO_S1_STRAIGHT_CHORD_ANGLE);
}

let x = c * (1.0 - 0.25 * other);
let y = other * (1.0 - 0.25 * c);

GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * f64::sqrt(x * y)))
}
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct GeoR3Vector {
x: f64,
y: f64,
z: f64,
}

impl Add for GeoR3Vector {
type Output = GeoR3Vector;

fn add(self, ov: GeoR3Vector) -> GeoR3Vector {
GeoR3Vector {
x: self.x + ov.x,
y: self.y + ov.y,
z: self.z + ov.z,
}
}
}

impl GeoR3Vector {
fn sub(self, ov: GeoR3Vector) -> GeoR3Vector {
GeoR3Vector {
x: self.x - ov.x,
y: self.y - ov.y,
z: self.z - ov.z,
}
}

fn norm2(&self) -> f64 {
self.dot(*self)
}

fn dot(&self, ov: GeoR3Vector) -> f64 {
self.x * ov.x + self.y * ov.y + self.z * ov.z
}
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct GeoS2Point {
geo_r3_vector: GeoR3Vector,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct GeoS2Cap {
center: GeoS2Point,
radius_sq: f64,
}

impl GeoS2Cap {
fn new(center: GeoS2Point, radius: GeoS1ChordAngle) -> GeoS2Cap {
GeoS2Cap {
center,
radius_sq: radius.0 * radius.0,
}
}

fn intersects(&self, other: GeoS2Cap) -> bool {
if self.radius_sq < 0.0 || other.radius_sq < 0.0 {
return false;
}

let dist_sq = geo_s2_chord_angle_between_points(self.center, other.center).0;
self.radius_sq + other.radius_sq >= dist_sq
}

fn is_empty(&self) -> bool {
self.radius_sq < 0.0
}
}

fn geo_s2_chord_angle_between_points(x: GeoS2Point, y: GeoS2Point) -> GeoS1ChordAngle {
GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, x.geo_r3_vector.sub(y.geo_r3_vector).norm2()))
}

