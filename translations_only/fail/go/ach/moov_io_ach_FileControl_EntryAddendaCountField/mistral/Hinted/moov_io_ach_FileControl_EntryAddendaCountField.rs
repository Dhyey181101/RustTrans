

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: [&str; 94] = ["0"; 94];

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
    // ... other fields ...
}

impl MoovIoAchFileControl {
    fn numeric_field(&self, max: u32) -> String {
        let s = self.entry_addenda_count.to_string();
        let l = s.len() as u32;
        if l > max {
            s[(l - max) as usize..].to_string()
        } else {
            let m = max - l;
            let pad = MoovIoAchConverters::get_pad(m as usize);
            pad.chars().rev().chain(s.chars()).collect()
        }
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn get_pad(n: usize) -> String {
        ZEROS[..n].iter().cloned().collect()
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

impl fmt::Display for MoovIoAchFileControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.numeric_field(8))
    }
}

