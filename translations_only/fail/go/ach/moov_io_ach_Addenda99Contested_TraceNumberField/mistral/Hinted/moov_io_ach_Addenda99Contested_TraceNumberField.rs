

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    trace_number: String,
    converters: Box<Converters>,
}

impl Addenda99Contested {
    fn new(trace_number: String, converters: Converters) -> Addenda99Contested {
        Addenda99Contested {
            trace_number,
            converters: Box::new(converters),
        }
    }
}

struct Converters {}

impl Converters {
    fn convert(&self, _input: &str) -> String {
        ZERO.to_string()
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda99Contested [trace_number={}, converters=<...>]", self.trace_number)
    }
}

