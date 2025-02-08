
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda98 {
    original_dfi: String,
    // ... other fields ...
    converters: Box<Converters98>,
}

struct Converters98;

impl Addenda98 {
    fn new() -> Addenda98 {
        Addenda98 {
            original_dfi: String::new(),
            converters: Box::new(Converters98),
        }
    }
}
