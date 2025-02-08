
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct Addenda99 {
    pub original_dfi: String,
    pub addenda_information: String,
    pub trace_number: String,
}

impl Addenda99 {
    pub fn original_dfi_field(&self) -> String {
        Converters {}.string_field(&self.original_dfi, 8)
    }
}

pub struct Converters {}

impl Converters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = &STRING_ZEROS[&m];
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i));
        }
        out
    };
}
