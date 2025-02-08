

use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchAddenda99Contested {
    original_entry_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn original_entry_trace_number_field(&self) -> String {
        string_field(&self.original_entry_trace_number, 15)
    }
}

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).cloned().unwrap_or_else(|| "0".repeat(m as usize));
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

struct MoovIoAchConverters;

