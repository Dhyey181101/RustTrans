

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

#[derive(Debug)]
struct Converters;

struct Addenda17 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<Converters>,
}

impl Addenda17 {
}

impl fmt::Display for Addenda17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda17 [entry_detail_sequence_number={}, moov_io_ach_converters={:?}]",
               self.entry_detail_sequence_number, self.moov_io_ach_converters)
    }
}

