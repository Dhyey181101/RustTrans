
use std::f64::consts::PI;

fn geo_s2_chord_angle_between_points(x: geo_r3_vector, y: geo_r3_vector) -> geo_s1_chord_angle {
    geo_s1_chord_angle(f64::min(4.0, x.sub(y).norm2()))
}

#[derive(Debug, Clone, Copy)]
struct geo_r3_vector {
    x: f64,
    y: f64,
    z: f64,
}

impl geo_r3_vector {
    fn sub(self, ov: geo_r3_vector) -> geo_r3_vector {
        geo_r3_vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(self) -> f64 {
        self.dot(self)
    }

    fn dot(self, ov: geo_r3_vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

#[derive(Debug, Clone, Copy)]
struct geo_s1_chord_angle(f64);

impl geo_s1_chord_angle {
    fn to_degrees(self) -> f64 {
        self.0 * 180.0 / PI
    }
}
