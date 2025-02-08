
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda98Refused {
    original_dfi: String,
    // ... other fields ...
    converters: Box<Converters>,
}

struct Converters;

impl Addenda98Refused {
    fn new(original_dfi: String) -> Addenda98Refused {
        Addenda98Refused {
            original_dfi,
            converters: Box::new(Converters),
        }
    }
}
