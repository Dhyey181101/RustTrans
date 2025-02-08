

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

#[derive(Debug)]
struct Converters;

struct Addenda99Contested {
    date_original_entry_returned: String,
    moov_io_ach_converters: Box<Converters>,
}

impl Addenda99Contested {
    fn new() -> Self {
        Addenda99Contested {
            date_original_entry_returned: String::new(),
            moov_io_ach_converters: Box::new(Converters),
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested [")?;
        write!(f, "date_original_entry_returned={:?}, ", self.date_original_entry_returned)?;
        write!(f, "moov_io_ach_converters={:?}", self.moov_io_ach_converters)?;
        write!(f, "]")
    }
}

