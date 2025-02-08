
use std::f64;

const GEO_S1_RADIAN: f64 = 1.;

fn geo_r3_vector_angle(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    let cross_product = geo_r3_vector_cross(v.to_owned(), ov.to_owned());
    f64::atan2(geo_r3_vector_norm(&cross_product), geo_r3_vector_dot(v, ov)) * GEO_S1_RADIAN
}

fn geo_r3_vector_cross(v: geo_r3_Vector, ov: geo_r3_Vector) -> geo_r3_Vector {
    geo_r3_Vector {
        x: v.y * ov.z - v.z * ov.y,
        y: v.z * ov.x - v.x * ov.z,
        z: v.x * ov.y - v.y * ov.x,
    }
}

fn geo_r3_vector_norm(v: &geo_r3_Vector) -> f64 {
    f64::sqrt(geo_r3_vector_dot(v, v))
}

fn geo_r3_vector_dot(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}

#[derive(Copy, Clone)]
struct geo_r3_Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_Vector {
    fn to_owned(&self) -> geo_r3_Vector {
        geo_r3_Vector { x: self.x, y: self.y, z: self.z }
    }
}
