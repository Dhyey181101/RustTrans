
use std::io::{self, BufReader, Read};

struct GeoS2CellID(u64);

struct GeoS2Decoder {
    r: BufReader<Box<dyn Read>>,
    err: Option<io::Error>,
}

impl GeoS2CellID {
    fn decode(&mut self, r: Box<dyn Read>) -> io::Result<()> {
        let mut decoder = GeoS2Decoder {
            r: geo_s2_as_byte_reader(r),
            err: None,
        };
        decode(self, &mut decoder);
        match decoder.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

fn geo_s2_as_byte_reader(r: Box<dyn Read>) -> BufReader<Box<dyn Read>> {
    BufReader::new(r)
}

fn decode(ci: &mut GeoS2CellID, d: &mut GeoS2Decoder) {
    *ci = GeoS2CellID(read_uint64(d));
}

fn read_uint64(d: &mut GeoS2Decoder) -> u64 {
    if d.err.is_some() {
        return 0;
    }
    let mut x = 0u64;
    d.err = match d.r.read_exact(&mut x.to_le_bytes()) {
        Ok(_) => None,
        Err(e) => Some(e),
    };
    x
}
