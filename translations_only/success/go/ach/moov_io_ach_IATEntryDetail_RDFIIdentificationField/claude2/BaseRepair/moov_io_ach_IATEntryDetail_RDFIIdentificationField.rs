
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
}

impl MoovIoAchIatEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(self.rdfi_identification.clone(), 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: String, max: u32) -> String {
        let mut pad = String::new();
        for _ in 0..max {
            pad.push('0');
        }
        if s.len() > max as usize {
            format!("{}", &s[..max as usize])
        } else {
            format!("{}{}", &pad[..max as usize - s.len()], s)
        }
    }
}
