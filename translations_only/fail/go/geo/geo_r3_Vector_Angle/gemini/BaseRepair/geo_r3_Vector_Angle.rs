
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct GeoR3Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeoR3Vector {
    pub fn angle(&self, ov: &GeoR3Vector) -> f64 {
        let cross = self.cross(ov);
        let norm = cross.norm();
        let dot = self.dot(ov);
        (norm / dot).atan() * PI
    }

    pub fn cross(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.y * ov.z - self.z * ov.y,
            y: self.z * ov.x - self.x * ov.z,
            z: self.x * ov.y - self.y * ov.x,
        }
    }

    pub fn norm(&self) -> f64 {
        (self.dot(self)).sqrt()
    }

    pub fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
