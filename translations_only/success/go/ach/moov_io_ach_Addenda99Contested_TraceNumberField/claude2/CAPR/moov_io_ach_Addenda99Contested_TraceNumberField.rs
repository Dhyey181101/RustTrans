
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda99Contested {
    trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn trace_number_field(&self) -> String {
        string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        "0".repeat(m as usize) + s
    }
}
