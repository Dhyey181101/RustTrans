

use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchFileControl {
    entry_hash: i32,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        self.converters.numeric_field(self.entry_hash, 10)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            String::from(&s[(l-max) as usize..])
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, "0");
            if let Some(pad) = pad {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m as usize) + &s
            }
        }
    }
}

fn get_pad_string(n: usize, c: &str) -> Option<String> {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out.get(&n).cloned()
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... format and write the struct fields to `f` ...
        Ok(())
    }
}

