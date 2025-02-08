

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    total_debit: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

impl MoovIoAchAdvBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        self.converters.numeric_field(self.total_debit, 20)
    }
}

struct MoovIoAchConverters {
    pad_strings: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = &self.pad_strings[&(m as usize)];
            return pad.repeat(m as usize) + &s;
        }
    }
}

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        let mut out = HashMap::new();
        for i in 0..=20 {
            out.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters { pad_strings: out }
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        Ok(())
    }
}

