
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99 {
    original_trace: String,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda99 {
    fn new() -> Addenda99 {
        Addenda99 {
            original_trace: String::new(),
            moov_io_ach_converters: Box::new(Converters),
        }
    }
}
