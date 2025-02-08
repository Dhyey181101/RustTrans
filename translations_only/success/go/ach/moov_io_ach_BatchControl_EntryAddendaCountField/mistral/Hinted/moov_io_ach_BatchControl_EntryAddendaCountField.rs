

use std::collections::HashMap;
use std::fmt;
use std::str;

const SERVICE_CLASS_CODE_MIXED: &str = "200";
const SERVICE_CLASS_CODE_CREDITS: &str = "220";
const SERVICE_CLASS_CODE_DEBITS: &str = "225";

struct MoovIoAchBatchControl {
    entry_addenda_count: i32,
    // ... other fields elided ...
    converters: Box<MoovIoAchConverters>,
}

struct MoovIoAchConverters;

impl MoovIoAchBatchControl {
    fn entry_addenda_count_field(&self) -> String {
        self.converters.numeric_field(self.entry_addenda_count, 6)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            let start = l - max;
            let mut result = String::new();
            for (i, c) in s.chars().enumerate() {
                if i >= start as usize {
                    result.push(c);
                }
            }
            result
        } else {
            let m = max - l;
            let pad = get_zeros(m as usize);
            format!("{}{}", pad, s)
        }
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push('0');
    }
    out
}

impl fmt::Display for MoovIoAchBatchControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        Ok(())
    }
}

