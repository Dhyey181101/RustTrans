
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchBatchControl {
    odfi_identification: String,
}

impl MoovIoAchBatchControl {
    fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.odfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            format!("{}{}", pad, s)
        }
    }
}
