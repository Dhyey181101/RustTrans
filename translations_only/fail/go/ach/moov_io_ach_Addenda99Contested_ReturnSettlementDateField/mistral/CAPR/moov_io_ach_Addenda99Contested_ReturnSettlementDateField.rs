
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Contested {
    return_settlement_date: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Contested {
    fn new() -> Addenda99Contested {
        Addenda99Contested {
            return_settlement_date: String::new(),
            moov_io_ach_converters: Converters,
        }
    }
}
