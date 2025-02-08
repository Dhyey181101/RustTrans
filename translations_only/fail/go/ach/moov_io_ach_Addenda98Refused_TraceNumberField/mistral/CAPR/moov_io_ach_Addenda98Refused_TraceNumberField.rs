
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda98Refused {
    trace_number: String,
    converters: Converters,
}

struct Converters;

impl Addenda98Refused {
    fn new(trace_number: String) -> Self {
        Addenda98Refused {
            trace_number,
            converters: Converters,
        }
    }
}
