
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Dishonored {
    trace_number: String,
    converters: Converters,
}

struct Converters;

impl Addenda99Dishonored {
    fn new(trace_number: String) -> Addenda99Dishonored {
        Addenda99Dishonored {
            trace_number,
            converters: Converters,
        }
    }
}
