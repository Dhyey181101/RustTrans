

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

#[derive(Debug)]
struct Converters;

struct Addenda99Dishonored {
    original_entry_trace_number: String,
    // ... other fields ...
    moov_io_ach_converters: Box<Converters>,
}

impl Addenda99Dishonored {
    // ... other methods ...
}

impl fmt::Display for Addenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "original_entry_trace_number: {:?}, moov_io_ach_converters: {:?}",
            self.original_entry_trace_number, self.moov_io_ach_converters
        )
    }
}

