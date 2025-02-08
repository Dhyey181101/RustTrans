
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Cap {
    center: Point,
    radius: f64,
}

#[derive(Copy, Clone)]
pub struct Point {
    vector: Vector,
}

#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub fn contains_point(cap: &Cap, p: &Point) -> bool {
    chord_angle_between_points(&cap.center, p) <= cap.radius
}

pub fn chord_angle_between_points(x: &Point, y: &Point) -> f64 {
    f64::min(4.0, x.vector.sub(&y.vector).norm2())
}

impl Vector {
    pub fn sub(&self, ov: &Vector) -> Vector {
        Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, ov: &Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

impl From<f64> for Vector {
    fn from(x: f64) -> Self {
        Vector { x, y: 0.0, z: 0.0 }
    }
}

impl From<Vector> for f64 {
    fn from(v: Vector) -> Self {
        v.x
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vector { x, y, z }
    }
}

impl From<Vector> for (f64, f64, f64) {
    fn from(v: Vector) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<f64> for Point {
    fn from(x: f64) -> Self {
        Point {
            vector: Vector::from(x),
        }
    }
}

impl From<Vector> for Point {
    fn from(v: Vector) -> Self {
        Point { vector: v }
    }
}

impl From<Point> for Vector {
    fn from(p: Point) -> Self {
        p.vector
    }
}

impl From<Point> for f64 {
    fn from(p: Point) -> Self {
        p.vector.x
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Point {
            vector: Vector::from((x, y, z)),
        }
    }
}

impl From<Point> for (f64, f64, f64) {
    fn from(p: Point) -> Self {
        (p.vector.x, p.vector.y, p.vector.z)
    }
}
