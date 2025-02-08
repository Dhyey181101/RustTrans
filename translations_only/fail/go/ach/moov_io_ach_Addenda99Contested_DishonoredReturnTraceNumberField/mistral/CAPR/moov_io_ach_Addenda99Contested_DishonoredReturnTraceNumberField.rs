
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda9 {
    data: HashMap<i32, String>,
}

impl Addenda9 {
    fn new() -> Addenda9 {
        Addenda9 {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &i32) -> Option<&String> {
        self.data.get(key)
    }
}

impl fmt::Display for Addenda9 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addenda9(\n")?;
        for (key, value) in &self.data {
            writeln!(f, "\t{}: {}\n", key, value)?;
        }
        write!(f, ")")
    }
}
