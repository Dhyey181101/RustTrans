
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct IATEntryDetail {
    pub rdfi_identification: String,
    pub check_digit: String,
}

impl IATEntryDetail {
    pub fn set_rdfi(&mut self, rdfi: &str) {
        let s = rdfi.chars().take(9).collect::<String>();
        self.rdfi_identification = s[..8].to_string();
        self.check_digit = s[8..9].to_string();
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
        let pad = &"0".repeat(m);
        format!("{}{}", pad, s)
    }

    pub fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

impl Converters {
    pub fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
        let mut out = HashMap::new();
        for i in 0..max {
            out.insert(i, zero.repeat(i));
        }
        out
    }
}

fn main() {
    let mut iat_ed = IATEntryDetail {
        rdfi_identification: "".to_string(),
        check_digit: "".to_string(),
    };
    iat_ed.set_rdfi("123456789");
    println!("{:?}", iat_ed);

    let converters = Converters {};
    let s = converters.string_field("12345", 10);
    println!("{}", s);

    let s = converters.parse_string_field(" 12345 ");
    println!("{}", s);

    let map = Converters::populate_map(10, "0");
    println!("{:?}", map);
}
