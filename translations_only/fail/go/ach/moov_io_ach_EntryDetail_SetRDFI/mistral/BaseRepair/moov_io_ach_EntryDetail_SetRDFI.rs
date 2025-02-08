

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let m = (max - ln) as usize;
            let pad = get_zeros(m);
            pad + s
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields elided ...
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str, c: &MoovIoAchConverters) -> &MoovIoAchEntryDetail {
        let s = c.string_field(rdfi, 9);
        self.rdfi_identification = c.parse_string_field(&s[..8]);
        self.check_digit = c.parse_string_field(&s[8..9]);
        self
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=94 {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}, CheckDigit: {}",
            self.rdfi_identification, self.check_digit
        )
    }
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        rdfi_identification: "".to_string(),
        check_digit: "".to_string(),
        // ... other fields elided ...
    };
    let rdfi = "123456789";
    let c = MoovIoAchConverters;
    ed.set_rdfi(rdfi, &c);
    println!("{}", ed);
}

