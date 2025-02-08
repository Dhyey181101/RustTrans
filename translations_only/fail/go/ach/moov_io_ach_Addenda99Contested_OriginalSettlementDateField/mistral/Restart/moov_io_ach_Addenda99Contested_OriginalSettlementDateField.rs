

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

#[derive(Debug)]
struct Converters;

struct Addenda99Contested {
    original_settlement_date: String,
    // ... other fields ...
    moov_io_ach_converters: Box<Converters>,
}

impl Addenda99Contested {
    // ... other methods ...
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "original_settlement_date: {:?}, moov_io_ach_converters: {:?}",
            self.original_settlement_date, self.moov_io_ach_converters
        )
    }
}

