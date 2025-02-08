

use std::fmt;
use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone, Default, PartialEq)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn norm2(&self) -> f64 {
        self.dot(*self)
    }

    fn dot(&self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Point {
    vector: Vector3D,
}

impl Point {
    fn sub(&self, other: Point) -> Vector3D {
        self.vector - other.vector
    }
}

#[derive(Copy, Clone, Default)]
struct Cap {
    center: Point,
    radius: ChordAngle,
}

impl Cap {
    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    fn add_point(&mut self, point: Point) {
        if self.is_empty() {
            self.center = point;
            self.radius = ChordAngle(0.0);
            return;
        }

        let new_rad = ChordAngle::from_norm2(point.sub(self.center).norm2());
        if new_rad.0 > self.radius.0 {
            self.radius = new_rad;
        }
    }
}

#[derive(Copy, Clone, Default)]
struct ChordAngle(f64);

impl ChordAngle {
    fn from_norm2(norm2: f64) -> ChordAngle {
        ChordAngle(norm2.min(4.0))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.partial_cmp(&other.0)?)
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({})", self.vector)
    }
}

impl fmt::Display for Cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cap({}, {})", self.center, self.radius.0)
    }
}

impl fmt::Display for ChordAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChordAngle({})", self.0)
    }
}

