
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0".to_string())
});

struct MoovIoAchAddenda99 {
    trace_number: String,
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            match MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)) {
                Some(pad) => pad.to_owned() + s,
                None => "0".repeat(m as usize) + s,
            }
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: String) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

