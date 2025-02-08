
use std::io::{self, Write};

struct GeoS2CellID(u64);

struct GeoS2Encoder<W: Write> {
    w: W,
    err: Option<io::Error>,
}

impl GeoS2CellID {
    fn encode(&self, w: &mut impl Write) -> io::Result<()> {
        let mut encoder = GeoS2Encoder {
            w,
            err: None,
        };
        encode(self, &mut encoder);
        match encoder.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

fn encode(ci: &GeoS2CellID, e: &mut GeoS2Encoder<impl Write>) {
    write_uint64(e, ci.0);
}

fn write_uint64(e: &mut GeoS2Encoder<impl Write>, x: u64) {
    if e.err.is_some() {
        return;
    }
    let buf = x.to_le_bytes();
    e.err = e.w.write_all(&buf).err();
}
