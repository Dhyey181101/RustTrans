
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    date_original_entry_returned: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Addenda99Contested {
        Addenda99Contested {
            date_original_entry_returned: String::from(""),
            moov_io_ach_converters: Converters,
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested")
    }
}
