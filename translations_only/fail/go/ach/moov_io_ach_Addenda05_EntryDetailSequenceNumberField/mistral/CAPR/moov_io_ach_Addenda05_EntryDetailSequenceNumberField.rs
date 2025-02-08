
use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct Addenda05 {
    entry_detail_sequence_number: i32,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda05 {
}
