
use std::fmt;
use std::collections::HashMap;
use std::boxed::Box;

const ZEROS: &str = "0";

struct Addenda1 {
    value: i32,
}

impl fmt::Display for Addenda1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

struct MyStruct {
    map: HashMap<i32, Box<Addenda1>>,
}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: Addenda1) {
        self.map.insert(key, Box::new(value));
    }

    fn display(&self) {
        for (k, v) in &self.map {
            println!("Key: {}, Value: {}", k, v);
        }
    }
}
