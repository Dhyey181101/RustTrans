
use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    trace_number: String,
    converters: Box<Converters>,
}

impl Addenda99Contested {
    fn new(trace_number: String) -> Addenda99Contested {
        Addenda99Contested {
            trace_number,
            converters: Box::new(Converters::new()),
        }
    }

    fn add_converter(&mut self, key: String, value: Box<dyn Converter>) {
        self.converters.add_converter(key, value);
    }

    fn convert(&self, input: &str) -> String {
        self.converters.convert(&self.trace_number, input)
    }
}

struct Converters {
    converter_map: HashMap<String, Box<dyn Converter>>,
}

impl Converters {
    fn new() -> Converters {
        Converters {
            converter_map: HashMap::new(),
        }
    }

    fn add_converter(&mut self, key: String, value: Box<dyn Converter>) {
        self.converter_map.insert(key, value);
    }

    fn convert(&self, trace_number: &str, input: &str) -> String {
        let converter = self.converter_map.get(trace_number).unwrap();
        converter.convert(input)
    }
}

trait Converter {
    fn convert(&self, input: &str) -> String;
}

struct IdentityConverter;

impl Converter for IdentityConverter {
    fn convert(&self, input: &str) -> String {
        input.to_string()
    }
}
