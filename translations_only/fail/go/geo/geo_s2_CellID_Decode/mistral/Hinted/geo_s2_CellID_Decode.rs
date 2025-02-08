
use std::io::{self, Read};
use std::convert::TryInto;

struct GeoS2Decoder {
    r: Box<dyn Read>,
    err: io::Error,
}

impl GeoS2Decoder {
    fn new(r: Box<dyn Read>) -> Self {
        GeoS2Decoder { r, err: io::Error::new(io::ErrorKind::Other, "") }
    }

    fn read_uint64(&mut self) -> Result<u64, io::Error> {
        let mut buf = [0u8; 8];
        match self.r.read_exact(&mut buf) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        let r = u64::from_le_bytes(buf);
        Ok(r)
    }
}
