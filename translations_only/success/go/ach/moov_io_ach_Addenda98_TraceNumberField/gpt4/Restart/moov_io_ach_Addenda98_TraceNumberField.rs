
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> Box<HashMap<usize, String>> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    Box::new(out)
}

struct MoovIoAchAddenda98 {
    trace_number: String,
}

impl MoovIoAchAddenda98 {
    fn trace_number_field(&self) -> String {
        let ln = self.trace_number.chars().count();
        if ln > 15 {
            return self.trace_number.chars().take(15).collect();
        }

        let m = 15 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                return pad.clone() + &self.trace_number;
            }
        }

        // slow path
        "0".repeat(m) + &self.trace_number
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
