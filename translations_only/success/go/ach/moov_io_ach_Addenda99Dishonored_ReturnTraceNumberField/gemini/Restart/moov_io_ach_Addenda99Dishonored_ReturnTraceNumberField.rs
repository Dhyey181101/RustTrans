
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct MoovIoAchAddenda99Dishonored {
    pub return_trace_number: String,
    pub return_settlement_date: String,
    pub return_reason_code: String,
    pub addenda_information: String,
    pub trace_number: String,
}

impl MoovIoAchAddenda99Dishonored {
    pub fn return_trace_number_field(&self) -> String {
        self.string_field(&self.return_trace_number, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = MOOv_IO_ACH_STRING_ZEROS.get(&m).unwrap_or_else(|| {
            panic!("Failed to get pad for m: {}", m)
        });
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref MOOv_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
