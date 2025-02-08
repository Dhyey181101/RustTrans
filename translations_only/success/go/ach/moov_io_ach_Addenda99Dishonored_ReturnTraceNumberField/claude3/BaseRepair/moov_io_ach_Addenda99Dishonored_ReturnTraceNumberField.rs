
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn moov_io_ach_string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        if let Some(zeros) = &MOOV_IO_ACH_STRING_ZEROS {
            if let Some(pad) = zeros.get(&m) {
                return pad.to_owned() + s;
            }
        }
    }

    "0".repeat(m) + s
}

struct MoovIoAchAddenda99Dishonored {
    return_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_trace_number_field(&self) -> String {
        moov_io_ach_string_field(&self.return_trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        moov_io_ach_string_field(s, max)
    }
}
