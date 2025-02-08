
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Cap {
    pub center: Point,
    pub radius: ChordAngle,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub vector: Vector,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ChordAngle(pub f64);

impl Cap {
    pub fn centroid(&self) -> Point {
        if self.is_empty() {
            return Point { vector: Vector { x: 0.0, y: 0.0, z: 0.0 } };
        }
        let r = 1.0 - 0.5 * self.height();
        Point {
            vector: self.center.vector.mul(r * self.area()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }

    pub fn height(&self) -> f64 {
        0.5 * self.radius.0
    }

    pub fn area(&self) -> f64 {
        2.0 * PI * f64::max(0.0, self.height())
    }
}

impl Vector {
    pub fn mul(&self, m: f64) -> Vector {
        Vector {
            x: m * self.x,
            y: m * self.y,
            z: m * self.z,
        }
    }
}
