
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda98 {
    trace_number: String,
    // ... other fields ...
}

struct Converters;

impl Addenda98 {
    fn new(trace_number: String) -> Addenda98 {
        Addenda98 { trace_number }
    }

    fn trace_number(&self) -> &String {
        &self.trace_number
    }

    // ... other methods ...
}

impl fmt::Display for Addenda98 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.trace_number)
    }
}
