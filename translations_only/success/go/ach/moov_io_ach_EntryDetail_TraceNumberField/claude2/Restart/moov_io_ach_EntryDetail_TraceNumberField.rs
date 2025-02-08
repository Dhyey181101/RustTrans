
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| populate_map(94, "0".to_string()));

struct MoovIoAchEntryDetail {
    trace_number: String,
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
        let pad = MOOV_IO_ACH_STRINGZEROS.get(&(m as usize)).cloned().unwrap_or_else(|| "0".to_string());
        format!("{}{}", pad, s)
    }
}

fn populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

