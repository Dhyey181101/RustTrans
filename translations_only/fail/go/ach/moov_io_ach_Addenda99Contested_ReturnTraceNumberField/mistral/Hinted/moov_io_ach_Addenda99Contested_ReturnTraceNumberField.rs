
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    return_trace_number: String,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Self {
        Addenda99Contested {
            return_trace_number: String::from(""),
            moov_io_ach_converters: Box::new(Converters),
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested")
    }
}
