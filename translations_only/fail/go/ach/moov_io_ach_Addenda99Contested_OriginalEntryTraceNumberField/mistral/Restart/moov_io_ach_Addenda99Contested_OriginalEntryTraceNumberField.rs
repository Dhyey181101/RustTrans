
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    original_entry_trace_number: String,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Addenda99Contested {
        Addenda99Contested {
            original_entry_trace_number: String::new(),
            moov_io_ach_converters: Box::new(Converters),
        }
    }
}
