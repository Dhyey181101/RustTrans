

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

#[derive(Debug)]
struct Converters {}

impl Converters {
    fn new() -> Converters {
        Converters {}
    }
}

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

struct Contested {
    amount: i32,
    reason: String,
}

impl Contested {
    fn new(amount: i32, reason: String) -> Contested {
        Contested { amount, reason }
    }
}

impl fmt::Display for Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contested {{ amount: {}, reason: {} }}", self.amount, self.reason)
    }
}

struct Addenda9 {
    contested: Option<Contested>,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 {
            contested: None,
        }
    }

    fn set_contested(&mut self, contested: Contested) {
        self.contested = Some(contested);
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Addenda99Contested {{ trace_number: {}, converters: {:?} }}",
            self.trace_number, self.converters
        )
    }
}

