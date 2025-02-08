
use std::f64::consts::PI;

struct GeoS1ChordAngle(f64);

fn tan(c: &GeoS1ChordAngle) -> f64 {
    sin(c) / cos(c)
}

fn sin(c: &GeoS1ChordAngle) -> f64 {
    (sin2(c) as f64).sqrt()
}

fn sin2(c: &GeoS1ChordAngle) -> f64 {
    c.0 * (1.0 - 0.25 * c.0)  
}

fn cos(c: &GeoS1ChordAngle) -> f64 {
    1.0 - 0.5 * c.0
}

