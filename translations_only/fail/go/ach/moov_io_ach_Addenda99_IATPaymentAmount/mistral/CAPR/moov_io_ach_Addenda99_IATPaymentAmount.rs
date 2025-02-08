
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99 {
    addenda_information: String,
}

struct Converters;

impl Addenda99 {
    fn new() -> Addenda99 {
        Addenda99 {
            addenda_information: String::new(),
        }
    }

    fn from_go_code(addenda_info: &str) -> Addenda99 {
        let mut addenda99 = Addenda99::new();
        addenda99.addenda_information = addenda_info.to_string();
        addenda99
    }

    fn to_string(&self) -> String {
        self.addenda_information.clone()
    }
}

impl fmt::Display for Addenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
