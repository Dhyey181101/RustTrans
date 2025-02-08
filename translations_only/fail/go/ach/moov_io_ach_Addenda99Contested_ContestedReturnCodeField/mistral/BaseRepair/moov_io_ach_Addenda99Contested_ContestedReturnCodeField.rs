

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
        self.converters.string_field(&self.contested_return_code, 3)
    }
}

struct Converters {
    map: HashMap<usize, String>,
}

impl Converters {
    fn new() -> Self {
        let mut out = Converters {
            map: HashMap::new(),
        };
        for i in 0..=10 {
            out.map.insert(i, "0".repeat(i));
        }
        out
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            self.map[&m].clone() + s
        }
    }
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting implementation ...
        Ok(())
    }
}

