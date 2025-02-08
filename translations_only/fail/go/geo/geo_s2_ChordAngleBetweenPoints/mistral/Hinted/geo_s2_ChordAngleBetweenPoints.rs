

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::f64;

#[derive(Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul for Vector {
    type Output = f64;
    fn mul(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Vector {
    fn norm2(&self) -> f64 {
        *self * *self
    }
}

struct Point {
    vector: Vector,
}

type ChordAngle = f64;

fn geo_s2_chord_angle_between_points(x: Point, y: Point) -> ChordAngle {
    let sub = Vector {
        x: x.vector.x - y.vector.x,
        y: x.vector.y - y.vector.y,
        z: x.vector.z - y.vector.z,
    };
    f64::min(4.0, sub.norm2())
}

fn main() {
    let v1 = Vector {
        x: 2.172974619038387e-282,
        y: 1.0410273767887174e-42,
        z: 5.076076235e-315,
    };
    let v2 = Vector {
        x: 5.643420714543095e-62,
        y: 2.71615478903e-311,
        z: 7.34567821899417e-67,
    };
    let p1 = Point { vector: v1 };
    let p2 = Point { vector: v2 };
    println!("{}", geo_s2_chord_angle_between_points(p1, p2));
}

