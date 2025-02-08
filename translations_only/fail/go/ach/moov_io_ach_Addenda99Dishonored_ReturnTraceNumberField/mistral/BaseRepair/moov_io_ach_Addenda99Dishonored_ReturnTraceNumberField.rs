

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Dishonored {
    return_trace_number: String,
    // ... other fields ...
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda99Dishonored {
    // ... other methods ...
}

impl fmt::Display for Addenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Addenda99Dishonored [ return_trace_number = {}, moov_io_ach_converters = {} ]",
            self.return_trace_number, self.moov_io_ach_converters
        )
    }
}

impl fmt::Display for Converters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Converters")
    }
}
