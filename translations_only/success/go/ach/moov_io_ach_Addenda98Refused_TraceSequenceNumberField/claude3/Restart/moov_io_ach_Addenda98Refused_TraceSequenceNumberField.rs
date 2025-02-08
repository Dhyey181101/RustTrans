
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

fn moov_io_ach_get_string_zeros() -> &'static HashMap<usize, String> {
    unsafe {
        if MOOV_IO_ACH_STRING_ZEROS.is_none() {
            MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
        }
        MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap()
    }
}

struct MoovIoAchAddenda98Refused {
    trace_sequence_number: String,
}

impl MoovIoAchAddenda98Refused {
    fn trace_sequence_number_field(&self) -> String {
        moov_io_ach_string_field(&self.trace_sequence_number, 7)
    }
}

fn moov_io_ach_string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    let zeros = moov_io_ach_get_string_zeros();
    if let Some(pad) = zeros.get(&m) {
        pad.to_owned() + s
    } else {
        "0".repeat(m) + s
    }
}
