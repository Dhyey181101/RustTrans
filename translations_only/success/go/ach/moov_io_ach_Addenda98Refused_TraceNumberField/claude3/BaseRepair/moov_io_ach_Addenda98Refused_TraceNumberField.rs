

use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, Box<str>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, Box<str>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, Box::from(zero.repeat(i)));
    }
    out
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        let converter = MoovIoAchConverters;
        converter.string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(ref map) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = map.get(&m) {
                    return pad.to_string() + s;
                }
            }
        }

        "0".repeat(m) + s
    }
}

