
use lazy_static::lazy_static;

use std::collections::HashMap;

pub struct MoovIoAchAddenda99Dishonored {
    pub return_settlement_date: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_settlement_date_field(&self) -> String {
        MoovIoAchConverters {}.string_field(&self.return_settlement_date, 3)
    }
}

pub struct MoovIoAchConverters;

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&"0".repeat(m as usize));
        let binding = "0".repeat(m as usize);
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
