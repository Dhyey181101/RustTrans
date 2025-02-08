
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoovIoAchAddenda99 {
    pub original_trace: String,
}

impl MoovIoAchAddenda99 {
    pub fn original_trace_field(&self) -> String {
        self.string_field(&self.original_trace, 15)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = moov_io_ach_string_zeros.get(&m).unwrap();
        return format!("{}{}", pad, s);
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str("0").unwrap().repeat(i));
        }
        out
    };
}
