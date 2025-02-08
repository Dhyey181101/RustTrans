
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';
const SPACE: char = ' ';

struct Addenda99 {
    date_of_death: String,
    converters: Converters,
}

struct Converters;

impl Addenda99 {
    fn new() -> Addenda99 {
        Addenda99 {
            date_of_death: String::new(),
            converters: Converters,
        }
    }
}
