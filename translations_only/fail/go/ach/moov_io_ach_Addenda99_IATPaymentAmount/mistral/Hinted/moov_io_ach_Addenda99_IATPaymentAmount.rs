
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99 {
    addenda_information: String,
}

struct Converters;

impl Addenda99 {
    fn new(addenda_information: String) -> Addenda99 {
        Addenda99 {
            addenda_information,
        }
    }

    fn convert_to_ascii(&self) -> String {
        self.addenda_information.chars()
            .map(|c| format!("{} {}", ZERO, c))
            .collect()
    }
}

impl fmt::Display for Addenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.convert_to_ascii())
    }
}
