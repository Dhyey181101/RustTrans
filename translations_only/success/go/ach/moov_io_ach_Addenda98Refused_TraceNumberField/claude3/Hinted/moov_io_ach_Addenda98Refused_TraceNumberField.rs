
use std::collections::HashMap;
use std::str;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        return s.chars().take(max as usize).collect();
    }

    let m = (max - ln) as usize;
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        let pad = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m);
        if let Some(pad) = pad {
            return pad.to_owned() + s;
        }
    }

    "0".repeat(m) + s
}

struct MoovIoAchAddenda98Refused {
    trace_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn trace_number_field(&self) -> String {
        moov_io_ach_string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        moov_io_ach_string_field(s, max)
    }
}
