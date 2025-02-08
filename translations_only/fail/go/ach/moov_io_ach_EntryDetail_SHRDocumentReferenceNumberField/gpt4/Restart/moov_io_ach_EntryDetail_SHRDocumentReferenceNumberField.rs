
use std::collections::HashMap;

static mut MOOV_IO_ACH_STRING_ZEROS: Option<Box<HashMap<usize, String>>> = None;

struct MoovIoAchEntryDetail {
    identification_number: String,
}

impl MoovIoAchEntryDetail {
    fn shr_document_reference_number_field(&self) -> String {
        let s = &self.identification_number[4..15];
        let max = 11;
        let ln = s.chars().count();
        if ln > max {
            return s.chars().take(max).collect();
        }

        let m = max - ln;
        let pad = unsafe {
            MOOV_IO_ACH_STRING_ZEROS
                .as_ref()
                .unwrap()
                .get(&m)
                .unwrap()
                .clone()
        };
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..=max {
        out.insert(i, zero.repeat(i));
    }
    out
}

fn main() {
    unsafe {
        MOOV_IO_ACH_STRING_ZEROS = Some(Box::new(moov_io_ach_populate_map(94, "0")));
    }
}
