
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| {
    moov_io_ach_populate_map(94, "0")
});

struct MoovIoAchEntryDetail {
    identification_number: String,
}

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: &str) {
        self.identification_number += &MoovIoAchConverters::string_field(s.to_string(), 11);
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = max - ln;
            MOOV_IO_ACH_STRING_ZEROS.get(&(m as usize)).cloned().unwrap_or_else(|| "0".repeat(m as usize)) + &s
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

