

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

#[derive(Debug)]
struct Addenda98Refused {
    original_trace: String,
}

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
    addenda_9_map: HashMap<String, Addenda98Refused>,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 {
            addenda_9_map: HashMap::new(),
        }
    }

    fn add(&mut self, key: String, addenda98refused: Addenda98Refused) {
        self.addenda_9_map.insert(key, addenda98refused);
    }

    fn get(&self, key: &String) -> Option<&Addenda98Refused> {
        self.addenda_9_map.get(key)
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda9({:?})", self.addenda_9_map)
    }
}

