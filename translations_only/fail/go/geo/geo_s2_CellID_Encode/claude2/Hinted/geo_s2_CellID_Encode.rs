
use std::io;
use std::io::Write;

struct GeoS2CellId(u64);

struct GeoS2Encoder<'a> {
    w: &'a mut dyn Write,
    err: Option<io::Error>,
}

impl GeoS2CellId {
    fn encode(&self, e: &mut GeoS2Encoder) {
        write_u64_to_encoder(e, self.0);
    }

    fn encode_to_writer(&self, w: &mut dyn Write) -> Result<(), io::Error> {
        let mut e = GeoS2Encoder { w, err: None };
        self.encode(&mut e);
        Err(e.err.unwrap_or(io::ErrorKind::Other.into()))
    }
}

fn write_u64_to_encoder(e: &mut GeoS2Encoder, x: u64) {
    if e.err.is_none() {
        let _ = e.w.write_all(&x.to_le_bytes());
        e.err = e.w.write_all(&x.to_le_bytes()).err();
    }
}

