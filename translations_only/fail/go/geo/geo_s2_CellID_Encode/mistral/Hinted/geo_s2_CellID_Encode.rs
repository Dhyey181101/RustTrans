
use std::io::{self, Write};
use std::error::Error;
use std::fmt;

struct GeoS2Encoder {
    w: Box<dyn Write + 'static>,
    err: Option<Box<dyn Error + 'static>>,
}

struct GeoS2CellID(u64);

impl GeoS2CellID {
    fn to_str(&self) -> String {
        format!("{}", self.0)
    }
}

impl fmt::Display for GeoS2CellID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
