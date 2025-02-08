

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            let mut m = (s.len() as u32) - max;
            let mut start = s.len() - (m as usize);
            if start < 0 {
                start = 0;
            }
            s[start..].to_string()
        } else {
            let m = max as i32 - s.len() as i32;
            let pad = get_pad_string(m);
            pad + &s
        }
    }
}

fn get_pad_string(n: i32) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i as usize));
    }
    out[&n].clone()
}

struct MoovIoAchAddenda17 {
    sequence_number: i32,
    // ... other fields ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda17 {
    fn sequence_number_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.sequence_number, 4)
    }
}

fn main() {
    // ...
}

