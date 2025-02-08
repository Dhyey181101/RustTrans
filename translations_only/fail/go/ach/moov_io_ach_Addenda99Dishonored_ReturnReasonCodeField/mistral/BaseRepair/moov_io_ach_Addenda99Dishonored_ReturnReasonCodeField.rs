
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Dishonored {
    return_reason_code: String,
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
            "Addenda99Dishonored [return_reason_code: '{}']",
            self.return_reason_code
        )
    }
}
