
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl GeoR3Vector {
    fn sub(&self, ov: &GeoR3Vector) -> GeoR3Vector {
        GeoR3Vector {
            x: self.x - ov.x,
            y: self.y - ov.y,
            z: self.z - ov.z,
        }
    }

    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoS1ChordAngle(f64);

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, x.geo_r3_vector.sub(&y.geo_r3_vector).norm2()))
}

fn interior_contains_point(c: &GeoS2Cap, p: &GeoS2Point) -> bool {
    is_full(c) || geo_s2_chord_angle_between_points(&c.center, p).0 < c.radius.0
}

fn is_full(c: &GeoS2Cap) -> bool {
    c.radius.0 == GEO_S1_STRAIGHT_CHORD_ANGLE.0
}
