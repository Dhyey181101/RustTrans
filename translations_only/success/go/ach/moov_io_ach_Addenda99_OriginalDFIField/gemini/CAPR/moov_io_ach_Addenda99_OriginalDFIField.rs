
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Addenda99 {
    pub original_dfi: String,
}

impl Addenda99 {
    pub fn original_dfi_field(&self) -> String {
        self.string_field(&self.original_dfi, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = STRING_ZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_utf8(vec![b'0'; i]).unwrap());
        }
        out
    };
}
