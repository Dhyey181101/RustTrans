

use std::io::{Read, Error as IoError, ErrorKind, Cursor};
use std::mem::swap;

struct GeoS2Cap {
    center: (f64, f64, f64),
    radius: f64,
}

struct GeoS2Decoder {
    inner: Cursor<Vec<u8>>,
}

impl GeoS2Decoder {
    fn new(data: Vec<u8>) -> Self {
        GeoS2Decoder {
            inner: Cursor::new(data),
        }
    }

    fn read_float64(&mut self) -> Result<f64, IoError> {
        let mut buf = [0; 8];
        self.inner.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
}

fn decode(c: &mut GeoS2Cap, d: &mut GeoS2Decoder) -> Result<(), IoError> {
    c.center.0 = d.read_float64()?;
    c.center.1 = d.read_float64()?;
    c.center.2 = d.read_float64()?;
    c.radius = geo_s1_ChordAngle(d.read_float64()?);
    Ok(())
}

fn geo_s1_ChordAngle(radians: f64) -> f64 {
    // Implementation not provided
    unimplemented!()
}

