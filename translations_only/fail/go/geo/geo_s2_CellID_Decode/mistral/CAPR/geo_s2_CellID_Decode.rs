
use std::io::{self, Read, Cursor};
use std::convert::TryInto;

struct GeoS2Decoder {
    r: Box<dyn Read>,
    err: Option<io::Error>,
}

impl GeoS2Decoder {
    fn read_uint64(&mut self) -> u64 {
        match &self.err {
            Some(e) => return e.raw_os_error().unwrap_or(0) as u64,
            None => {
                let mut buf = [0; 8];
                self.r.read_exact(&mut buf).expect("Failed to read buffer");
                u64::from_le_bytes(buf)
            }
        }
    }
}
