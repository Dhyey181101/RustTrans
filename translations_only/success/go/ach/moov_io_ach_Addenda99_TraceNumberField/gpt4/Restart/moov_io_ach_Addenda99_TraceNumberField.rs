
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda99 {
    trace_number: String,
}

impl MoovIoAchAddenda99 {
    fn trace_number_field(&self) -> String {
        moov_io_ach_string_field(&self.trace_number, 15)
    }
}

fn moov_io_ach_string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s.chars().take(max).collect();
    }

    let m = max - ln;
    if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
        return format!("{}{}", pad, s);
    }

    // slow path
    "0".repeat(m) + s
}

fn main() {}
