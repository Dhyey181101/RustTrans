
use std::f64::consts::PI;

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);

struct GeoS2Cap {
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    // Assuming GeoR3Vector is an empty struct as per Go code
}

struct GeoR3Vector {}

#[derive(Copy, Clone)]
struct GeoS1ChordAngle(f64);

#[derive(Copy, Clone)]
struct GeoS1Angle(f64);

fn radius(c: GeoS2Cap) -> GeoS1Angle {
    angle(c.radius)
}

fn angle(c: GeoS1ChordAngle) -> GeoS1Angle {
    if c.0 < 0.0 {
        return GeoS1Angle(-1.0 * GEO_S1_RADIAN.0);
    }
    if is_infinity(c) {
        return geo_s1_inf_angle();
    }
    GeoS1Angle(2.0 * (0.5 * c.0).sqrt().asin())
}

fn is_infinity(c: GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(f64::INFINITY)
}

fn main() {
    // Example usage
    let cap = GeoS2Cap {
        radius: GeoS1ChordAngle(1.2291734130893592e-147),
    };
    let result = radius(cap);
    println!("{}", result.0);
}
