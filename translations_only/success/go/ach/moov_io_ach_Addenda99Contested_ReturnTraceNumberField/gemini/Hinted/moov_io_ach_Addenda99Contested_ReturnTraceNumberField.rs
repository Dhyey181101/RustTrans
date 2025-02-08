
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub struct MoovIoAchAddenda99Contested {
    pub return_trace_number: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn return_trace_number_field(&self) -> String {
        self.string_field(&self.return_trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = "0".repeat(m);
        let pad = MOOV_IO_ACH_STRING_ZEROS.get(&m).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            let binding = "0".repeat(i);
            out.insert(i, binding);
        }
        out
    };
}
