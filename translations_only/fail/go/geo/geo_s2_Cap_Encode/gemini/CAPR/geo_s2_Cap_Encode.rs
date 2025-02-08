
use std::io::{Error, Write};

pub struct Cap {
    center: Point,
    radius: ChordAngle,
}

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

pub struct ChordAngle(pub f64);

pub fn encode_cap<W: Write>(cap: &Cap, w: &mut W) -> Result<(), Error> {
    w.write_all(&cap.center.x.to_be_bytes())?;
    w.write_all(&cap.center.y.to_be_bytes())?;
    w.write_all(&cap.center.z.to_be_bytes())?;
    w.write_all(&cap.radius.0.to_be_bytes())
}
