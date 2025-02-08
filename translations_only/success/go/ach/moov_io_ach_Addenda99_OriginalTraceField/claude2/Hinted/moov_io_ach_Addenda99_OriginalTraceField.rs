

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99 {
    original_trace: String,
}

impl MoovIoAchAddenda99 {
    fn original_trace_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.original_trace, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut result = String::new();
            for _ in 0..(max - ln) {
                result.push('0');
            }
            result + s
        }
    }
}

