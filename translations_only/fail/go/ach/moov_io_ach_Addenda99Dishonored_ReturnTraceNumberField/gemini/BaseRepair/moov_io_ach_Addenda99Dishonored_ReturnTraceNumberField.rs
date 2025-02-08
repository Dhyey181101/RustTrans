
use std::collections::HashMap;

pub struct MoovIoAchAddenda99Dishonored {
    pub return_trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_trace_number_field(&self) -> &str {
        &self.return_trace_number
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap();

        format!("{}{}", pad, s)
    }
}

lazy_static::lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
