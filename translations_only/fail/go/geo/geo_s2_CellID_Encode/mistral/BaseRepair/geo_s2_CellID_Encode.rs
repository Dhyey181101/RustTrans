
use std::io::{self, Write};
use std::error::Error;

struct GeoS2Encoder {
    w: Box<dyn Write + 'static>,
    err: Option<Box<dyn Error + 'static>>,
}

impl GeoS2Encoder {
    fn new(w: Box<dyn Write + 'static>) -> GeoS2Encoder {
        GeoS2Encoder {
            w,
            err: None,
        }
    }
}
