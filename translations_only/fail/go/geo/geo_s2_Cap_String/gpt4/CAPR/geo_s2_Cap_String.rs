
use std::f64::consts::PI;

const GEO_S1_RADIAN: GeoS1Angle = 1.0;
const GEO_S1_DEGREE: GeoS1Angle = (PI / 180.0) * GEO_S1_RADIAN;

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    geo_r3_vector: Box<GeoR3Vector>,
}

#[derive(Debug)]
struct GeoR3Vector {}

type GeoS1ChordAngle = f64;
type GeoS1Angle = f64;

fn geo_s1_inf_angle() -> GeoS1Angle {
    f64::INFINITY
}

fn geo_s1_chord_angle_is_infinity(c: GeoS1ChordAngle) -> bool {
    c.is_infinite()
}

fn geo_s1_chord_angle_angle(c: GeoS1ChordAngle) -> GeoS1Angle {
    if c < 0.0 {
        return -1.0 * GEO_S1_RADIAN;
    }
    if geo_s1_chord_angle_is_infinity(c) {
        return geo_s1_inf_angle();
    }
    2.0 * (0.5 * f64::sqrt(c)).asin()
}

fn geo_s2_cap_radius(c: &GeoS2Cap) -> GeoS1Angle {
    geo_s1_chord_angle_angle(c.radius)
}

fn geo_s2_cap_string(c: &GeoS2Cap) -> String {
    format!("[Center={:?}, Radius={}]", c.center.geo_r3_vector, geo_s1_angle_degrees(geo_s2_cap_radius(c)))
}

fn geo_s1_angle_degrees(a: GeoS1Angle) -> f64 {
    a / GEO_S1_DEGREE
}
