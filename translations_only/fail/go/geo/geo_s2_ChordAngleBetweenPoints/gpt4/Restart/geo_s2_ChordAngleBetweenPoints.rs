
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
}

struct GeoS1ChordAngle(f64);

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, sub(&x.geo_r3_vector, &y.geo_r3_vector).norm2()))
}

fn sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> Box<GeoR3Vector> {
    Box::new(GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    })
}

impl GeoR3Vector {
    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
