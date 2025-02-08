
use std::collections::HashMap;

use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<usize, String>> = Lazy::new(|| HashMap::new());

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
        let mut result = String::new();
        let ln = s.len() as u32;
        if ln > max {
            result.push_str(&s[..max as usize]);
        } else {
            for _ in 0..(max - ln) {
                result.push('0');
            }
            result.push_str(s);
        }
        result
    }
}

