
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
        MoovIoAchConverters::string_field(&self.trace_number, 15)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: usize) -> String {
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
                    return format!("{}{}", pad, s);
                }
            }
        }

        // slow path
        "0".repeat(m) + s
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
