
use std::collections::HashMap;

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = moov_io_ach_populate_map(94, "0");
}

struct MoovIoAchEntryDetail {
    identification_number: String,
}

impl MoovIoAchEntryDetail {
    fn shr_document_reference_number_field(&self) -> String {
        let ln = self.identification_number[4..15].chars().count();
        if ln > 11 {
            return self.identification_number[4..15].chars().take(11).collect();
        }

        let m = 11 - ln;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return format!("{}{}", pad, &self.identification_number[4..15]);
        }

        format!("{}{}", "0".repeat(m), &self.identification_number[4..15])
    }
}

fn moov_io_ach_populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::with_capacity(max);
    for i in 0..max {
        out.insert(i, zero.repeat(i));
    }
    out
}

#[macro_use]
extern crate lazy_static;
