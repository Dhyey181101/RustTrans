
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn norm(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn angle(&self, other: Vector3) -> f64 {
        let cross_norm = self.cross(other).norm();
        let dot_product = self.dot(other);
        let angle = cross_norm.atan2(dot_product);
        angle * (PI / 180.0)
    }
}

fn main() {
    let v1 = Vector3 {
        x: 5.53291521591885e-222,
        y: 1.3034542755917937e+190,
        z: 1.4594065939148333e-205,
    };
    let v2 = Vector3 {
        x: 1.3034546188472022e+190,
        y: 1.3034546188519292e+190,
        z: 4.2851764543512724e-260,
    };

    let angle = v1.angle(v2);
    println!("{}", angle);
}
