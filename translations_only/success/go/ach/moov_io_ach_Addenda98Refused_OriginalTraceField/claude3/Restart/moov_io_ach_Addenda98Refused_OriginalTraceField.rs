

use std::collections::HashMap;
use std::str;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| moov_io_ach_populate_map(94, "0".to_string()));

struct MoovIoAchAddenda98Refused {
    original_trace: String,
}

impl MoovIoAchAddenda98Refused {
    fn original_trace_field(&self) -> String {
        let converter = Box::new(MoovIoAchConverters {});
        converter.string_field(&self.original_trace, 15)
    }
}

struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.to_string() + s;
        }

        "0".repeat(m) + s
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

