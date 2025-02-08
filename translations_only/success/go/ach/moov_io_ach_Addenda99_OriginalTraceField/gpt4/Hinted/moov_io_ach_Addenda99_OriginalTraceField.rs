
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

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
    original_trace: String,
}

impl MoovIoAchAddenda99 {
    fn original_trace_field(&self) -> String {
        let ln = self.original_trace.chars().count();
        if ln > 15 {
            return self.original_trace.chars().take(15).collect();
        }

        let m = 15 - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + &self.original_trace;
        }

        "0".repeat(m) + &self.original_trace
    }
}

fn main() {}
