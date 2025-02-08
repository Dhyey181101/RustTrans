

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchEntryDetail {
    trace_number: String,
    // other fields omitted
}

impl MoovIoAchEntryDetail {
    fn trace_number_field(&self) -> String {
        string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &str, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        "0".repeat(m as usize) + s
    }
}

