
use std::f64;
use std::ops::{Mul, Add};

#[derive(Debug, Clone)]
struct geo_r3_PreciseVector {
    x: Box<f64>,
    y: Box<f64>,
    z: Box<f64>,
}

#[derive(Debug, Clone, Copy)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_PreciseVector {
    fn vector(&self) -> geo_r3_Vector {
        geo_r3_Vector {
            x: *self.x,
            y: *self.y,
            z: *self.z,
        }.normalize()
    }
}

impl geo_r3_Vector {
    fn normalize(&self) -> geo_r3_Vector {
        let n2 = self.norm2();
        if n2 == 0.0 {
            return geo_r3_Vector { x: 0.0, y: 0.0, z: 0.0 };
        }
        self.mul(1.0 / n2.sqrt())
    }

    fn norm2(&self) -> f64 {
        self.dot(*self)
    }

    fn dot(&self, ov: geo_r3_Vector) -> f64 {
        (self.x * ov.x) + (self.y * ov.y) + (self.z * ov.z)
    }

    fn mul(&self, m: f64) -> geo_r3_Vector {
        geo_r3_Vector {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

fn main() {
    let invalid_input_1 = geo_r3_PreciseVector {
        x: Box::new(3.3843378940342868e+125),
        y: Box::new(f64::NAN),
        z: Box::new(f64::NAN),
    };
    println!("Example 0: {:?}", invalid_input_1.vector());

    let invalid_input_2 = geo_r3_PreciseVector {
        x: Box::new(-2.2226497953256543e+307),
        y: Box::new(f64::NAN),
        z: Box::new(f64::NAN),
    };
    println!("Example 1: {:?}", invalid_input_2.vector());
}
