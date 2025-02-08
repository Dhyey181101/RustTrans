

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct Addenda99Contested {
    dishonored_return_trace_number: String,
    // ... other fields ...
    converters: Converters,
}

impl Addenda99Contested {
    fn dishonored_return_trace_number(&self) -> String {
        self.string_field(self.dishonored_return_trace_number.clone(), 15)
    }
}

struct Converters;

impl Addenda99Contested {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[0..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_pad_string(m);
        pad + &s
    }
}

fn get_pad_string(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..100 {
        out.insert(i, "0".repeat(i));
    }

    out[&n].clone()
}

impl fmt::Display for Addenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DishonoredReturnTraceNumber: {}, ...",
            self.dishonored_return_trace_number()
        )
    }
}

fn main() {
    let addenda = Addenda99Contested {
        dishonored_return_trace_number: "123456789012345".to_string(),
        converters: Converters,
    };

    println!("{}", addenda);
}

