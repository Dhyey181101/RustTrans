

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
    // ... other fields and methods ...
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::new_instance().numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchConverters {
    fn new_instance() -> MoovIoAchConverters {
        MoovIoAchConverters {}
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, "0");
            format!("{}{}", pad, s)
        }
    }
}

fn get_pad_string(n: usize, c: &str) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement formatting for MoovIoAchFileControl
        write!(f, "Entry Addenda Count: {}", self.entry_addenda_count)
    }
}

