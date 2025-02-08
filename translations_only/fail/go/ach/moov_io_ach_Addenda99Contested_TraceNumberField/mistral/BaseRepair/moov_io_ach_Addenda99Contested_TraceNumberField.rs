
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    trace_number: String,
    converters: Converters,
}

impl Addenda99Contested {
    fn new(trace_number: String, converters: Converters) -> Addenda99Contested {
        Addenda99Contested {
            trace_number,
            converters,
        }
    }

    fn trace_number(&self) -> &String {
        &self.trace_number
    }

    fn converters(&self) -> &Converters {
        &self.converters
    }
}

struct Converters {
    converter_map: HashMap<String, Box<dyn Fn(&str) -> String + Send + Sync>>,
}

impl Converters {
    fn new() -> Converters {
        Converters {
            converter_map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, converter: Box<dyn Fn(&str) -> String + Send + Sync>) {
        self.converter_map.insert(key, converter);
    }

    fn get(&self, key: &str) -> Option<&Box<dyn Fn(&str) -> String + Send + Sync>> {
        self.converter_map.get(key)
    }
}

fn main() {
    let mut converters = Converters::new();
    converters.insert("converter1".to_string(), Box::new(|s: &str| s.to_string()));

    let addenda99_contested = Addenda99Contested::new("123456789".to_string(), converters);

    let converter = addenda99_contested.converters().get("converter1").unwrap();
    println!("Converted: {}", converter(&ZERO));
}
