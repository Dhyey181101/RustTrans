

use std::f64;

const GEO_S1_RADIAN: f64 = 1.;

fn geo_r3_vector_norm(v: &geo_r3_Vector) -> f64 {
f64::sqrt(geo_r3_vector_dot(&v.x, &v.y, &v.z))
}

fn geo_r3_vector_cross(v: geo_r3_Vector, ov: geo_r3_Vector) -> geo_r3_Vector {
geo_r3_Vector {
x: v.y * ov.z - v.z * ov.y,
y: v.z * ov.x - v.x * ov.z,
z: v.x * ov.y - v.y * ov.x,
}
}

fn geo_r3_vector_dot(x: &f64, y: &f64, z: &f64) -> f64 {
x * y + x * z + y * z
}

fn geo_r3_vector_angle(v: &geo_r3_Vector, ov: &geo_r3_Vector) -> f64 {
f64::atan2(
geo_r3_vector_cross(*v, *ov).x,
geo_r3_vector_dot(&v.x, &v.y, &v.z),
) * GEO_S1_RADIAN
}

#[derive(Copy, Clone)]
struct geo_r3_Vector {
x: f64,
y: f64,
z: f64,
}

type geo_s1_Angle = f64;

