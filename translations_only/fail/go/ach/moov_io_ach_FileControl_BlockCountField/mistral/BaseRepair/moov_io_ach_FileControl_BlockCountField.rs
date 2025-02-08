
use std::collections::HashMap;
use std::fmt;
use std::str;

struct MoovIoAchFileControl {
    block_count: i32,
    // ... other fields ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn block_count_field(&self) -> String {
        self.converters.numeric_field(self.block_count, 6)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l-max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = get_pad_string(m as usize, "0");
            if let Some(pad) = pad {
                pad + &s
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
    out.get(&(n as usize)).cloned()
}

// Implement Display trait for MoovIoAchFileControl
impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlockCountField: {}", self.block_count_field())
    }
}
