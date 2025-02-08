
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Addenda99 {
    pub addenda_information: String,
}

impl Addenda99 {
    pub fn iat_payment_amount(&mut self, s: &str) {
        self.addenda_information = self.string_field(s, 10);
    }
}

impl Addenda99 {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = STRING_ZEROS.get(&m).unwrap();
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
