
use std::collections::HashMap;

struct MoovIoAchAddenda02 {
    trace_number: String,
}

impl MoovIoAchAddenda02 {
    fn trace_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = "0".repeat(m as usize);
            format!("{}{}", pad, s)
        }
    }
}
