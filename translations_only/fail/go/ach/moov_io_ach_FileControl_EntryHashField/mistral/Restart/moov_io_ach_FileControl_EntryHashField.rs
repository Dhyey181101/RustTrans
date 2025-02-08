

use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchFileControl {
    entry_hash: i32,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    numeric_field_cache: HashMap<(i32, u32), String>,
}

impl MoovIoAchFileControl {
    fn entry_hash_field(&self) -> String {
        let mut converters = self.converters.clone();
        converters.numeric_field(self.entry_hash, 10)
    }
}

impl Clone for MoovIoAchConverters {
    fn clone(&self) -> Self {
        MoovIoAchConverters {
            numeric_field_cache: self.numeric_field_cache.clone(),
        }
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&mut self, n: i32, max: u32) -> String {
        if let Some(cached) = self.numeric_field_cache.get(&(n, max)) {
            return cached.clone();
        }

        let s = n.to_string();
        let l = s.len() as u32;
        let result = if l > max {
            String::from(&s[(l-max) as usize..])
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, "0");
            if let Some(pad) = pad {
                format!("{}{}", pad, s)
            } else {
                "0".repeat(m as usize) + &s
            }
        };

        self.numeric_field_cache.insert((n, max), result.clone());
        result
    }
}

fn get_pad_string(n: usize, c: &str) -> Option<String> {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out.get(&n).cloned()
}

// Implement Display trait for MoovIoAchFileControl
impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntryHashField: {}", self.entry_hash_field())
    }
}

