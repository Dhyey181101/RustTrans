
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    return_trace_number: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Self {
        Addenda99Contested {
            return_trace_number: String::new(),
            moov_io_ach_converters: Converters,
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested")
    }
}
