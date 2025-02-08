
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<HashMap<usize, String>> = None;

struct MoovIoAchADVBatchControl {
    odfi_identification: String,
}

impl MoovIoAchADVBatchControl {
    fn odfi_identification_field(&self) -> String {
        self.string_field(&self.odfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            s.chars().take(max).collect()
        } else {
            let m = max - ln;
            let pad = unsafe {
                MOOV_IO_ACH_STRING_ZEROS.as_ref().unwrap().get(&m).unwrap().clone()
            };
            format!("{}{}", pad, s)
        }
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(moov_io_ach_populate_map(94, "0"));
    }
}
