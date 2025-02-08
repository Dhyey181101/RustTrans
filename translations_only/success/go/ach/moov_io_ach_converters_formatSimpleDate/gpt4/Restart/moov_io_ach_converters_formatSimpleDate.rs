
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = moov_io_ach_populate_map(94, "0");
}

fn moov_io_ach_populate_map(max: u32, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::with_capacity(max as usize);
    for i in 0..max {
        out.insert(i, zero.repeat(i as usize));
    }
    out
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn format_simple_date(&self, s: &str) -> String {
        if s.is_empty() {
            return self.string_field(s, 6);
        }
        s.to_string()
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s.chars().take(max as usize).collect();
        }

        // Pad with preallocated string
        let m = max - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, s);
        }
        // slow path
        "0".repeat(m as usize) + s
    }
}

#[macro_use]
extern crate lazy_static;

fn main() {}
