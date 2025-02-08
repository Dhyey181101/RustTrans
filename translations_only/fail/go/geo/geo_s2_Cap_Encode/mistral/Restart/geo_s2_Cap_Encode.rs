

use std::boxed::Box;
use std::error::Error;
use std::io;
use std::io::Write;

struct GeoS2Encoder<W: Write> {
    w: W,
    err: Option<Box<dyn Error>>,
}

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

impl<W: Write> GeoS2Encoder<W> {
    fn new(w: W) -> Self {
        GeoS2Encoder { w, err: None }
    }

    fn write_float64(&mut self, x: f64) -> io::Result<()> {
        if self.err.is_some() {
            return Ok(());
        }
        self.err = Some(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to write center.x",
        )));
        self.w.write_all(&x.to_le_bytes())
    }
}

impl GeoS2Cap {
    fn encode(&self, w: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let mut e = GeoS2Encoder::new(w);
        self.encode_internal(&mut e);
        if e.err.is_some() {
            return Err(e.err.take().unwrap());
        }
        Ok(())
    }

    fn encode_internal(&self, e: &mut GeoS2Encoder<&mut impl Write>) {
        e.write_float64(self.center.vector.x)
            .expect("Failed to write center.x");
        e.write_float64(self.center.vector.y)
            .expect("Failed to write center.y");
        e.write_float64(self.center.vector.z)
            .expect("Failed to write center.z");
        e.write_float64(self.radius as f64)
            .expect("Failed to write radius");
    }
}

impl GeoS2Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        GeoS2Point {
            vector: GeoR3Vector { x, y, z },
        }
    }
}

