
use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct Addenda12 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda12 {
    fn new() -> Addenda12 {
        Addenda12 {
            entry_detail_sequence_number: 0,
            moov_io_ach_converters: Converters,
        }
    }
}
