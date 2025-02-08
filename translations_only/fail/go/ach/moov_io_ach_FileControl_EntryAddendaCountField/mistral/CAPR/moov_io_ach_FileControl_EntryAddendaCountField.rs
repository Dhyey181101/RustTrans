

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
    // ... other fields omitted for brevity
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    pad_strings: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut out = HashMap::new();
        for i in 0..=255 {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { pad_strings: out }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = self.pad_strings.get(&(m as usize)).unwrap().clone();
            return pad + &s;
        }
    }
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.converters.numeric_field(self.entry_addenda_count, 7))
    }
}

fn main() {
    let fc = MoovIoAchFileControl {
        entry_addenda_count: 123,
        // ... other fields initialized
        converters: Box::new(MoovIoAchConverters::new()),
    };
    println!("{}", fc);
}

