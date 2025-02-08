
use std::io::Write;
use std::error::Error;

pub struct GeoS2Cap {
    pub center: GeoS2Point,
    pub radius: f64,
}

pub struct GeoS2Point {
    pub x: f64,
    pub y: f64, 
    pub z: f64
}

pub fn encode(cap: &GeoS2Cap, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    write_f64(writer, cap.center.x)?;
    write_f64(writer, cap.center.y)?;
    write_f64(writer, cap.center.z)?;
    write_f64(writer, cap.radius)
}

fn write_f64(writer: &mut dyn Write, n: f64) -> Result<(), Box<dyn Error>> {
    writer.write_all(&n.to_le_bytes())?;
    Ok(())
}
