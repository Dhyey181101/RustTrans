

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = Self::get_pad_string(m);
        pad + s
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99Contested {
    dishonored_return_trace_number: String,
    // ... other fields and their initialization
}

impl MoovIoAchAddenda99Contested {
    fn dishonored_return_trace_number_field(&self) -> String {
        self.string_field(&self.dishonored_return_trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda99Contested {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {:<15} TypeCode: {:<10} ContestedReturnCode: {:<10}",
            self.dishonored_return_trace_number,
            // ... other fields
            "",
            ""
        )
    }
}

fn main() {
    let addenda = MoovIoAchAddenda99Contested::default();
    println!("{}", addenda);
}

