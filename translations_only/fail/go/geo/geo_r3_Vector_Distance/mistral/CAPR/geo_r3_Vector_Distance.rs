
use std::f64;

fn geo_r3_vector_distance(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    geo_r3_vector_sub(v, ov).norm()
}

fn geo_r3_vector_sub(v: geo_r3_Vector, ov: geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn geo_r3_vector_norm(v: geo_r3_Vector) -> f64 {
    v.norm()
}

fn geo_r3_vector_dot(v: geo_r3_Vector, ov: geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_Vector {
    fn norm(&self) -> f64 {
        f64::sqrt(self.dot(*self))
    }

    fn dot(&self, ov: geo_r3_Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
