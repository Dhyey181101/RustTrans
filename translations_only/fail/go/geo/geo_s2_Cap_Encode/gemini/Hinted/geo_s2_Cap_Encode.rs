
use std::io::{Error, Write};

pub struct GeoS2Cap {
    pub center: GeoS2Point,
    pub radius: GeoS1ChordAngle,
}

pub struct GeoS2Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct GeoS1ChordAngle(pub f64);

pub fn encode_cap(cap: &GeoS2Cap, w: &mut dyn Write) -> Result<(), Error> {
    write_f64(w, cap.center.x)?;
    write_f64(w, cap.center.y)?;
    write_f64(w, cap.center.z)?;
    write_f64(w, cap.radius.0)
}

fn write_f64(w: &mut dyn Write, x: f64) -> Result<(), Error> {
    w.write_all(&x.to_be_bytes())
}
