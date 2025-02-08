
use std::f64::consts::PI;

pub fn geo_s2_chord_angle_between_points(x: geo_s2_point, y: geo_s2_point) -> geo_s1_chord_angle {
    geo_s1_chord_angle(f64::min(4.0, geo_r3_vector_sub(&x.geo_r3_vector, &y.geo_r3_vector).norm2()))
}

pub struct geo_r3_vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn geo_r3_vector_sub(ov: &geo_r3_vector, ov2: &geo_r3_vector) -> geo_r3_vector {
    geo_r3_vector {
        x: ov.x - ov2.x,
        y: ov.y - ov2.y,
        z: ov.z - ov2.z,
    }
}

impl geo_r3_vector {
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, ov: &geo_r3_vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

pub struct geo_s2_point {
    pub geo_r3_vector: geo_r3_vector,
}

pub struct geo_s1_chord_angle(pub f64);

