
use std::io::{Error, Write};

pub struct Cap {
    pub center: Point,
    pub radius: ChordAngle,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct ChordAngle(pub f64);

pub fn encode_cap(cap: &Cap, w: &mut dyn Write) -> Result<(), Error> {
    encode_float64(cap.center.x, w)?;
    encode_float64(cap.center.y, w)?;
    encode_float64(cap.center.z, w)?;
    encode_float64(cap.radius.0, w)
}

fn encode_float64(x: f64, w: &mut dyn Write) -> Result<(), Error> {
    w.write_all(&x.to_le_bytes())
}
