
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

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
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let pad = STRING_ZEROS.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<u32, String> = populate_map(94, "0");
}

fn populate_map(max: usize, zero: &str) -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i as u32, zero.repeat(i));
    }
    out
}
