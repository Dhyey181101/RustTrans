

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    return_reason_code: String,
    // ... other fields ...
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl fmt::Display for Converters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Converters")
    }
}

impl Addenda99Contested {
    // ... other methods ...
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Addenda99Contested [return_reason_code={}, moov_io_ach_converters={}]",
            self.return_reason_code, self.moov_io_ach_converters
        )
    }
}

