

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &str = "0";

struct MoovIoAchAdvBatchControl {
    total_debit: i32,
    // ... other fields elided ...
    moov_io_ach_converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters {
    debit_pad: String,
}

impl MoovIoAchAdvBatchControl {
    fn total_debit_entry_dollar_amount_field(&self) -> String {
        self.moov_io_ach_converters.numeric_field(self.total_debit, 20)
    }
}

impl MoovIoAchConverters {
    fn new() -> Self {
        let mut out = HashMap::new();
        for i in 0..=94 {
            out.insert(i, "0".repeat(i));
        }
        Self {
            debit_pad: out[&94].clone(),
        }
    }

    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let mut result = String::new();
            result.push_str(&s[start as usize..]);
            result
        } else {
            let m = max - l;
            let debit_pad = &self.debit_pad[..m as usize];
            format!("{}{}", debit_pad, s)
        }
    }
}

impl fmt::Display for MoovIoAchAdvBatchControl {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        Ok(())
    }
}

