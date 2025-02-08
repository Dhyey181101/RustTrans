

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

#[derive(Debug)]
struct Addenda98Refused {
    original_trace: String,
}

struct Converters;

impl Addenda98Refused {
    fn new(original_trace: String) -> Addenda98Refused {
        Addenda98Refused { original_trace }
    }
}

impl fmt::Display for Addenda98Refused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda98Refused({})", self.original_trace)
    }
}

struct Addenda9 {
    addenda_9: Option<Box<Addenda98Refused>>,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 {
            addenda_9: None,
        }
    }

    fn with_addenda_9(mut self, addenda_9: Addenda98Refused) -> Addenda9 {
        self.addenda_9 = Some(Box::new(addenda_9));
        self
    }

    fn addenda_9(&self) -> &Option<Box<Addenda98Refused>> {
        &self.addenda_9
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.addenda_9.as_ref().unwrap_or(&Box::new(Addenda98Refused::new("".to_string()))) {
            ref a => write!(f, "Addenda9({:?})", a),
        }
    }
}

