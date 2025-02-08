
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Addenda99 {
    pub addenda_information: String,
}

impl Addenda99 {
    pub fn iat_payment_amount(&mut self, s: &str) {
        self.addenda_information = string_field(s, 10);
    }
}

fn string_field(s: &str, max: usize) -> String {
    let ln = s.chars().count();
    if ln > max {
        return s[..max].to_string();
    }

    let m = max - ln;
    STRING_ZEROS.get(&m).unwrap().to_string() + s
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, String::from_str("0").unwrap().repeat(i));
        }
        out
    };
}
