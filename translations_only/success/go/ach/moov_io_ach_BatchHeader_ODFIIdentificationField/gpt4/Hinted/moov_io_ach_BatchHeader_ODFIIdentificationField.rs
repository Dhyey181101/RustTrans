
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchBatchHeader {
    odfi_identification: String,
}

impl MoovIoAchBatchHeader {
    fn odfi_identification_field(&self) -> String {
        let ln = self.odfi_identification.chars().count();
        if ln > 8 {
            return self.odfi_identification.chars().take(8).collect();
        }

        let m = 8 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
            }
            if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m) {
                return format!("{}{}", pad, self.odfi_identification);
            }
        }

        // slow path
        "0".repeat(m) + &self.odfi_identification
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
