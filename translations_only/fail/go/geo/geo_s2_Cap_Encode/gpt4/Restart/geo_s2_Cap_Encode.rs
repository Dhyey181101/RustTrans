
use std::io::{self, Write};

struct GeoS2Cap {
    center: GeoS2Point,
    radius: GeoS1ChordAngle,
}

struct GeoS2Point {
    vector: GeoR3Vector,
}

struct GeoR3Vector {
    x: f64,
    y: f64,
    z: f64,
}

type GeoS1ChordAngle = f64;

struct GeoS2Encoder<'a> {
    w: &'a mut dyn Write,
    err: Option<io::Error>,
}

fn encode(geo_s2_cap: &GeoS2Cap, w: &mut dyn Write) -> io::Result<()> {
    let mut e = GeoS2Encoder { w, err: None };
    encode_internal(&geo_s2_cap.center.vector.x, &geo_s2_cap.center.vector.y, &geo_s2_cap.center.vector.z, &geo_s2_cap.radius, &mut e);
    match e.err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn encode_internal(x: &f64, y: &f64, z: &f64, radius: &f64, e: &mut GeoS2Encoder) {
    write_float64(e, *x);
    write_float64(e, *y);
    write_float64(e, *z);
    write_float64(e, *radius);
}

fn write_float64(e: &mut GeoS2Encoder, x: f64) {
    if e.err.is_some() {
        return;
    }
    let mut buf = [0u8; 8];
    little_endian_write_f64(&mut buf, x);
    e.err = e.w.write_all(&buf).err();
}

fn little_endian_write_f64(buf: &mut [u8; 8], x: f64) {
    let bytes: [u8; 8] = x.to_le_bytes();
    buf.copy_from_slice(&bytes);
}
