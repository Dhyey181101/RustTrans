
use std::io::{self, Write};

struct GeoS2CellID(u64);

struct GeoS2Encoder<W: Write> {
    w: W,
    err: Option<io::Error>,
}

impl GeoS2CellID {
    fn encode(&self, w: impl Write) -> io::Result<()> {
        let mut encoder = GeoS2Encoder {
            w,
            err: None,
        };
        self.encode_internal(&mut encoder);
        match encoder.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    fn encode_internal(&self, e: &mut GeoS2Encoder<impl Write>) {
        e.write_uint64(self.0);
    }
}

impl<W: Write> GeoS2Encoder<W> {
    fn write_uint64(&mut self, x: u64) {
        if self.err.is_some() {
            return;
        }
        let bytes = x.to_le_bytes();
        self.err = self.w.write_all(&bytes).err();
    }
}
