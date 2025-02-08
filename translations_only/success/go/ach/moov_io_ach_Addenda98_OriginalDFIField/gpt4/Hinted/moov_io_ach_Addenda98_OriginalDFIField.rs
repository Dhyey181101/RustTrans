
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
    original_dfi: String,
}

impl MoovIoAchAddenda98 {
    fn original_dfi_field(&self) -> String {
        let ln = self.original_dfi.chars().count();
        if ln > 8 {
            return self.original_dfi.chars().take(8).collect();
        }

        let m = 8 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                return pad.clone() + &self.original_dfi;
            }
        }

        // slow path
        "0".repeat(m) + &self.original_dfi
    }
}

fn main() {
    // Initialization or other logic here
}
