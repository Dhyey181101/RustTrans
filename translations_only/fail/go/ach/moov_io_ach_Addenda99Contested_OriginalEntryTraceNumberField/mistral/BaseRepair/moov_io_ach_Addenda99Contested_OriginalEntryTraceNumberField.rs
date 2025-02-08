
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

struct Entry {
    addenda_records: Vec<Addenda99Contested>,
}

impl Entry {
    fn new() -> Entry {
        Entry {
            addenda_records: Vec::new(),
        }
    }
}

struct Format9 {
    entries: Vec<Entry>,
}

impl Format9 {
    fn new() -> Format9 {
        Format9 {
            entries: Vec::new(),
        }
    }
}

impl fmt::Display for Format9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Format9")
    }
}
