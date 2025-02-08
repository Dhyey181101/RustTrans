
const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);
const GEO_S1_MAX_LENGTH2: f64 = 4.0;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct GeoS1ChordAngle(f64);

impl GeoS2Cap {
    fn is_empty(&self) -> bool {
        self.radius.0 < 0.0
    }
}

fn interior_intersects(c: &GeoS2Cap, other: &GeoS2Cap) -> bool {
    if c.radius.0 <= 0.0 || other.is_empty() {
        return false;
    }

    c.radius.0 + other.radius.0 > geo_s2_chord_angle_between_points(&c.center, &other.center).0
}

fn add(c: GeoS1ChordAngle, other: GeoS1ChordAngle) -> GeoS1ChordAngle {
    if other.0 == 0.0 {
        return c;
    }

    if c.0 + other.0 >= GEO_S1_MAX_LENGTH2 {
        return GEO_S1_STRAIGHT_CHORD_ANGLE;
    }

    let x = c.0 * (1.0 - 0.25 * other.0);
    let y = other.0 * (1.0 - 0.25 * c.0);
    GeoS1ChordAngle(f64::min(GEO_S1_MAX_LENGTH2, x + y + 2.0 * f64::sqrt(x * y)))
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, sub(&x.vector, &y.vector).norm2()))
}

fn sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

impl GeoR3Vector {
    fn norm2(&self) -> f64 {
        self.dot(self)
    }

    fn dot(&self, ov: &GeoR3Vector) -> f64 {
        self.x * ov.x + self.y * ov.y + self.z * ov.z
    }
}
