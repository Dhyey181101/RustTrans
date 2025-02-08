
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

    fn addenda_information(&self) -> &String {
        &self.addenda_information
    }

    fn addenda_information_mut(&mut self) -> &mut String {
        &mut self.addenda_information
    }
}

impl fmt::Display for Addenda99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.addenda_information)
    }
}

fn main() {
    let mut addenda99 = Addenda99::new();
    addenda99.addenda_information_mut().push('1');
    println!("{}", addenda99);
}
