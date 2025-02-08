

use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        string_field(&self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &String, max: u32) -> String {
    let ln = s.len() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        let m = max - ln;
        let pad = "0".repeat(m as usize);
        format!("{}{}", pad, s)
    }
}

