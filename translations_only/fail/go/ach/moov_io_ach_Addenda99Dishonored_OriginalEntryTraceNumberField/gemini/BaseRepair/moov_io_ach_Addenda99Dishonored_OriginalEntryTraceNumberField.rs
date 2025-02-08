
use std::collections::HashMap;

pub struct MoovIoAchAddenda99Dishonored {
    pub original_entry_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn original_entry_trace_number_field(&self) -> &str {
        &self.original_entry_trace_number
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = moov_io_ach_string_zeros().get(&m).unwrap().to_string();
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, "0".repeat(i));
    }
    out
}
