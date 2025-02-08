
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Dishonored {
    return_reason_code: String,
    // ... other fields ...
    moov_io_ach_converters: Option<Box<Converters>>,
}

struct Converters;

impl Addenda99Dishonored {
    fn new() -> Self {
        Addenda99Dishonored {
            return_reason_code: String::new(),
            moov_io_ach_converters: None,
        }
    }
}
