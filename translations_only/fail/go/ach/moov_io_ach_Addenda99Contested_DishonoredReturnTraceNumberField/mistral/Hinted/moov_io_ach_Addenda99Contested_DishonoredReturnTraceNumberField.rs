
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda9 {
    data: i32,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 { data: 0 }
    }

    fn add(&mut self, num: i32) {
        self.data += num;
    }

    fn get(&self) -> i32 {
        self.data
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
