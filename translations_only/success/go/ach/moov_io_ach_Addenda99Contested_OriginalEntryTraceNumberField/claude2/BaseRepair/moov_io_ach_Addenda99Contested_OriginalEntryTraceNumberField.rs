
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchAddenda99Contested {
    original_entry_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    fn original_entry_trace_number_field(&self) -> String {
        Converters::string_field(
            self.original_entry_trace_number.clone(),
            15,
        )
    }
}

struct Converters;

impl Converters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            let pad = MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).cloned().unwrap_or_else(|| "0".repeat(m as usize));
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

