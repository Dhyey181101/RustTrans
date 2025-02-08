

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
    // ... other fields omitted for brevity
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = MoovIoAchConverters::get_pad_string(m as usize);
            return pad + &s;
        }
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let converters = MoovIoAchConverters; // create an instance of MoovIoAchConverters here
        write!(
            f,
            "{{entryAddendaCount: {}}}",
            converters.numeric_field(self.entry_addenda_count, 8)
        )
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        entry_addenda_count: 123,
        // ... other fields omitted for brevity
    };
    println!("{}", fc);
}

