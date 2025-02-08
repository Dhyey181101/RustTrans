
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        moov_io_ach_string_field(&self.trace_number, 15)
    }
}

fn moov_io_ach_string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

struct MoovIoAchConverters;

