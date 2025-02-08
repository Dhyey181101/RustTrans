
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchAddenda98 {
    original_trace: String,
}

impl MoovIoAchAddenda98 {
    fn original_trace_field(&self) -> String {
        let ln = self.original_trace.chars().count();
        if ln > 15 {
            return self.original_trace.chars().take(15).collect();
        }

        let m = 15 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(ref zeros) = MOOV_IO_ACH_STRING_ZEROS {
                if let Some(pad) = zeros.get(&m) {
                    return format!("{}{}", pad, self.original_trace);
                }
            }
        }

        // slow path
        "0".repeat(m) + &self.original_trace
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
