

use std::io::{self, Write};

fn encode_geo_s2_cell_id(ci: u64, w: &mut dyn Write) -> io::Result<()> {
    let mut e = geo_s2_encoder { w, err: None };
    e.write_uint64(ci)?;
    Ok(())
}

struct geo_s2_encoder<'a> {
    w: &'a mut dyn Write,
    err: Option<Box<io::Error>>,
}

impl<'a> geo_s2_encoder<'a> {
    fn write_uint64(&mut self, x: u64) -> io::Result<()> {
        if self.err.is_none() {
            let mut buf = [0u8; 8];
            buf.copy_from_slice(&x.to_be_bytes());
            match self.w.write(&buf) {
                Ok(_) => (),
                Err(e) => self.err = Some(Box::new(e)),
            }
        }
        if let Some(err) = self.err.as_ref() {
            Err(io::Error::new(io::ErrorKind::Other, "Error encoding uint64"))
        } else {
            Ok(())
        }
    }
}

type geo_s2_cell_id = u64;

