
use std::io::{self, BufReader, Read};

struct GeoS2CellID(u64);

struct GeoS2Decoder<R: Read> {
    r: R,
    err: Option<io::Error>,
}

impl GeoS2CellID {
    fn decode<R: Read>(&mut self, r: R) -> io::Result<()> {
        let mut d = GeoS2Decoder {
            r: geo_s2_as_byte_reader(r),
            err: None,
        };
        decode(self, &mut d);
        match d.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

fn geo_s2_as_byte_reader<R: Read>(r: R) -> BufReader<R> {
    BufReader::new(r)
}

fn decode(cell_id: &mut GeoS2CellID, d: &mut GeoS2Decoder<BufReader<impl Read>>) {
    let value = read_uint64(d);
    *cell_id = GeoS2CellID(value);
}

fn read_uint64<R: Read>(d: &mut GeoS2Decoder<BufReader<R>>) -> u64 {
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
