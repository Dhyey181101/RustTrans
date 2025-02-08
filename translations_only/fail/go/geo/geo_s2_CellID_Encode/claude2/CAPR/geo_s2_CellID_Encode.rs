
use std::io;
use std::io::Write;

struct GeoS2CellId(u64);

struct GeoS2Encoder<'a> {
    w: Box<dyn Write + 'a>,
    err: Option<io::Error>,
}

fn encode(cell_id: &GeoS2CellId, encoder: &mut GeoS2Encoder) {
    write_u64(encoder, cell_id.0 as u64);
}

fn encode_to_writer(cell_id: &GeoS2CellId, w: &mut dyn Write) -> Result<(), io::Error> {
    let mut encoder = GeoS2Encoder { w: Box::new(w), err: None };
    encode(cell_id, &mut encoder);
    Err(encoder.err.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "error")))  
}

fn write_u64(encoder: &mut GeoS2Encoder, x: u64) {
    if encoder.err.is_none() {
        let _ = encoder.w.write_all(&x.to_le_bytes());
        encoder.err = Some(write_all(&mut *encoder.w, &x.to_le_bytes()).unwrap_err());
    }
}

fn write_all(w: &mut dyn Write, buf: &[u8]) -> Result<(), io::Error> {
    w.write_all(buf).map_err(|e| e)
}

