
use std::f64::consts::PI;

const GEO_S1_RADIAN: GeoS1Angle = GeoS1Angle(1.0);

struct GeoS1ChordAngle(f64);

struct GeoS1Angle(f64);

fn angle(c: &GeoS1ChordAngle) -> GeoS1Angle {
    if c.0 < 0.0 {
        return GeoS1Angle(-1.0 * GEO_S1_RADIAN.0);
    }
    if is_infinity(c) {
        return geo_s1_inf_angle();
    }
    GeoS1Angle(2.0 * (0.5 * c.0.sqrt()).asin())
}

fn is_infinity(c: &GeoS1ChordAngle) -> bool {
    c.0.is_infinite()
}

fn geo_s1_inf_angle() -> GeoS1Angle {
    GeoS1Angle(f64::INFINITY)
}

fn main() {
    let examples = vec![
        5.83527705259544e-310f64,
        1.84716075850284e-99f64,
        1.9531286431911179e232f64,
        1.9531286431911024e232f64,
    ];

    for example in examples {
        let chord_angle = GeoS1ChordAngle(example);
        let result = angle(&chord_angle);
        match result.0 {
            x if x.is_nan() => println!("Input is invalid, crash gracefully"),
            _ => println!("{}", result.0),
        }
    }
}
