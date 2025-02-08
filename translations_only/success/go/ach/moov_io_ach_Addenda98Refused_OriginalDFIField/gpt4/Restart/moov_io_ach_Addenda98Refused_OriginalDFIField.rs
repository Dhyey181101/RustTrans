
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

struct MoovIoAchAddenda98Refused {
    original_dfi: String,
}

impl MoovIoAchAddenda98Refused {
    fn original_dfi_field(&self) -> String {
        let ln = self.original_dfi.chars().count();
        if ln > 8 {
            return self.original_dfi.chars().take(8).collect();
        }

        // Pad with preallocated string
        let m = 8 - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, self.original_dfi);
        }
        // slow path
        "0".repeat(m) + &self.original_dfi
    }
}

fn main() {}
