
use std::io::{self, Read};
use std::convert::TryInto;

struct GeoS2Decoder {
r: Box<dyn Read>,
}

impl GeoS2Decoder {
fn read_uint64(r: &mut Box<dyn Read>) -> Result<u64, io::Error> {
let mut buf = [0; 8];
r.read_exact(&mut buf).unwrap();
Ok(u64::from_le_bytes(buf))
}
}
