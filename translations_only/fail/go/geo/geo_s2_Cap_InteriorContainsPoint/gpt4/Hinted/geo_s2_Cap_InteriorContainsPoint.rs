
const GEO_S1_STRAIGHT_CHORD_ANGLE: GeoS1ChordAngle = GeoS1ChordAngle(4.0);

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(PartialEq, PartialOrd)]
struct GeoS1ChordAngle(f64);

fn interior_contains_point(c: &GeoS2Cap, p: &GeoS2Point) -> bool {
    is_full(c) || geo_s2_chord_angle_between_points(&c.center, p) < c.radius
}

fn is_full(c: &GeoS2Cap) -> bool {
    c.radius == GEO_S1_STRAIGHT_CHORD_ANGLE
}

fn geo_s2_chord_angle_between_points(x: &GeoS2Point, y: &GeoS2Point) -> GeoS1ChordAngle {
    GeoS1ChordAngle(f64::min(4.0, norm2(&sub(&x.geo_r3_vector, &y.geo_r3_vector))))
}

fn sub(v: &GeoR3Vector, ov: &GeoR3Vector) -> GeoR3Vector {
    GeoR3Vector {
        x: v.x - ov.x,
        y: v.y - ov.y,
        z: v.z - ov.z,
    }
}

fn norm2(v: &GeoR3Vector) -> f64 {
    dot(v, v)
}

fn dot(v: &GeoR3Vector, ov: &GeoR3Vector) -> f64 {
    v.x * ov.x + v.y * ov.y + v.z * ov.z
}
