
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    original_entry_trace_number: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Contested {
    fn new(
        original_entry_trace_number: String,
        moov_io_ach_converters: Converters,
    ) -> Addenda99Contested {
        Addenda99Contested {
            original_entry_trace_number,
            moov_io_ach_converters,
        }
    }
}
