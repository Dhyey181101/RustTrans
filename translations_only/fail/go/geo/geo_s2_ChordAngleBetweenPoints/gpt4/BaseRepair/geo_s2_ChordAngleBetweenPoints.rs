
struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct GeoS2Point {
    geo_r3_vector: GeoR3Vector,
}

struct GeoS1ChordAngle(f64);

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
