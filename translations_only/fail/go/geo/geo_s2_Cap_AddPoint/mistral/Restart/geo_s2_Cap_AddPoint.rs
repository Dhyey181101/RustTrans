

use std::fmt;

#[derive(Copy, Clone, Default, PartialEq)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn sub(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }
}

#[derive(Copy, Clone, Default)]
struct Point {
    vector: Vector3D,
}

impl Point {
    fn sub(&self, other: &Point) -> Vector3D {
        self.vector.sub(&other.vector)
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

    fn add_point(&mut self, p: Point) {
        if self.is_empty() {
            self.center = p;
            self.radius = ChordAngle(0.0);
            return;
        }

        let new_rad = ChordAngle::chord_angle_between_points(self.center, p);
        if new_rad.0 > self.radius.0 {
            self.radius = new_rad;
        }
    }
}

#[derive(Copy, Clone, Default)]
struct ChordAngle(f64);

impl ChordAngle {
    fn chord_angle_between_points(x: Point, y: Point) -> ChordAngle {
        let v = x.sub(&y).norm2();
        ChordAngle(v.min(4.0))
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
        write!(f, "Cap({}, {})", self.center, self.radius)
    }
}

impl fmt::Display for ChordAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChordAngle({})", self.0)
    }
}

fn main() {
    let mut c = Cap::default();
    let p = Point::default();
    c.add_point(p);
    println!("{}", c);
}

