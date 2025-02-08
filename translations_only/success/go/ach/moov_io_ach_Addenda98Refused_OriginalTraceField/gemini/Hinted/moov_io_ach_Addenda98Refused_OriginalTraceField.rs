
use lazy_static::lazy_static;

use std::collections::HashMap;

pub struct MoovIoAchAddenda98Refused {
    pub original_trace: String,
}

impl MoovIoAchAddenda98Refused {
    pub fn original_trace_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.original_trace, 15)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let binding = "0".repeat(m as usize);
        let pad = moov_io_ach_string_zeros.get(&(m as i32)).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<i32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
