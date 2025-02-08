
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    return_reason_code: String,
    moov_io_ach_converters: Box<Converters>,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Self {
        Addenda99Contested {
            return_reason_code: String::new(),
            moov_io_ach_converters: Box::new(Converters),
        }
    }
}
