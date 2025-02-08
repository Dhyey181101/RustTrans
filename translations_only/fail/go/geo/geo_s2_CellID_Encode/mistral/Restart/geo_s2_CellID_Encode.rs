
use std::io::{self, Write};
use std::error::Error;
use std::fmt;

struct GeoS2Encoder {
    w: Box<dyn Write + 'static>,
    err: Option<Box<dyn Error + 'static>>,
}

struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn encode(&self, encoder: &mut GeoS2Encoder) -> Result<(), Box<dyn Error + 'static>> {
        encoder.write_uint64(self.0)?;
        Ok(())
    }
}

impl GeoS2Encoder {
    fn new(w: Box<dyn Write + 'static>) -> Self {
        GeoS2Encoder { w, err: None }
    }

    fn write_uint64(&mut self, value: u64) -> io::Result<()> {
        self.w.write_all(&value.to_le_bytes())
    }
}

fn geos2_cell_id_encode(cell_id: &GeoS2CellID, encoder: &mut GeoS2Encoder) -> Result<(), Box<dyn Error + 'static>> {
    cell_id.encode(encoder)
}

