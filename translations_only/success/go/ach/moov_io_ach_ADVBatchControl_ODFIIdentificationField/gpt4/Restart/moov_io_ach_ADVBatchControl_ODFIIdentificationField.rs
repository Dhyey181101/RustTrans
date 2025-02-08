
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

struct MoovIoAchADVBatchControl {
    odfi_identification: String,
}

impl MoovIoAchADVBatchControl {
    fn odfi_identification_field(&self) -> String {
        let ln = self.odfi_identification.chars().count();
        if ln > 8 {
            return self.odfi_identification.chars().take(8).collect();
        }

        let m = 8 - ln;
        unsafe {
            if MOOV_IO_ACH_STRING_ZEROS.is_none() {
                MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
            }
            let pad = MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap();
            return pad.clone() + &self.odfi_identification;
        }
    }
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
