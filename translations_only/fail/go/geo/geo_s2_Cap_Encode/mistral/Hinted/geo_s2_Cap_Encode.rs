
use std::boxed::Box;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem::size_of;

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
        let mut buf = [0; size_of::<f64>()];
        unsafe {
            let ptr = buf.as_mut_ptr();
            let len = buf.len();
            (ptr as *mut f64).write(x);
            self.w.write_all(&buf[..len])?;
        }
        Ok(())
    }
}

impl GeoS2Cap {
    fn encode(&self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let mut encoder = GeoS2Encoder::new(w);
        self.encode_internal(&mut encoder)?;
        if encoder.err.is_some() {
            return Err(encoder.err.take().unwrap());
        }
        Ok(())
    }

    fn encode_internal(&self, encoder: &mut GeoS2Encoder<&mut dyn Write>) -> Result<(), Box<dyn Error>> {
        encoder.write_float64(self.center.vector.x)?;
        encoder.write_float64(self.center.vector.y)?;
        encoder.write_float64(self.center.vector.z)?;
        encoder.write_float64(self.radius)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("test.bin")?;
    let cap = GeoS2Cap {
        center: GeoS2Point {
            vector: GeoR3Vector { x: 1.0, y: 2.0, z: 3.0 },
        },
        radius: 4.0,
    };
    cap.encode(&mut file)?;
    Ok(())
}
