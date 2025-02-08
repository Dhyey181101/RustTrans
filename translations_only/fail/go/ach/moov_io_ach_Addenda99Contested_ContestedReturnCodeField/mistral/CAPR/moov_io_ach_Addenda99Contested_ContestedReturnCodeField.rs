

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    contested_return_code: String,
    // ... other fields ...
    converters: Box<Converters>,
}

impl Addenda99Contested {
    fn contested_return_code_field(&self) -> String {
        let m = 3 - self.contested_return_code.len();
        match self.converters.as_ref().map.get(&m) {
            Some(pad) => pad.to_string() + &self.contested_return_code,
            None => "0".repeat(3),
        }
    }
}

struct Converters {
    map: HashMap<usize, String>,
}

impl Converters {
    fn new() -> Converters {
        Converters {
            map: get_pad_map(),
        }
    }
}

fn get_pad_map() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..=10 {
        out.insert(i, ZERO.repeat(i));
    }
    out
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting implementation ...
        Ok(())
    }
}

