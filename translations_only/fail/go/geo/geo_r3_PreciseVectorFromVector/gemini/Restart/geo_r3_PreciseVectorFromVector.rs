
use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let norm = self.norm();
        Vector {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn angle_between(&self, other: &Vector) -> f64 {
        let dot = self.dot(other);
        let norm1 = self.norm();
        let norm2 = other.norm();
        dot / (norm1 * norm2)
    }

    pub fn rotate_around_x(&self, angle: f64) -> Vector {
        let sin = angle.sin();
        let cos = angle.cos();
        Vector {
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos,
        }
    }

    pub fn rotate_around_y(&self, angle: f64) -> Vector {
        let sin = angle.sin();
        let cos = angle.cos();
        Vector {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos,
        }
    }

    pub fn rotate_around_z(&self, angle: f64) -> Vector {
        let sin = angle.sin();
        let cos = angle.cos();
        Vector {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z,
        }
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

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);

        assert_eq!(v1.dot(&v2), 32.0);
        assert_eq!(v1.cross(&v2), Vector::new(-3.0, 6.0, -3.0));
        assert_eq!(v1.norm(), (14.0).sqrt());
        assert_eq!(v1.normalize(), Vector::new(1.0 / (14.0).sqrt(), 2.0 / (14.0).sqrt(), 3.0 / (14.0).sqrt()));
        assert_eq!(v1.angle_between(&v2), (PI / 2.0).cos());
        assert_eq!(v1.rotate_around_x(PI / 2.0), Vector::new(1.0, -3.0, 2.0));
        assert_eq!(v1.rotate_around_y(PI / 2.0), Vector::new(3.0, 2.0, -1.0));
        assert_eq!(v1.rotate_around_z(PI / 2.0), Vector::new(-2.0, 1.0, 3.0));
    }
}
