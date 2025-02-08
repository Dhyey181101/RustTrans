
use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.get_zeros(m);
            pad + s
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }

    fn get_zeros(&self, n: usize) -> String {
        let mut zeros = String::new();
        for _ in 0..n {
            zeros.push('0');
        }
        zeros
    }
}

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields
}

impl MoovIoAchAdvEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = MoovIoAchConverters.string_field(rdfi, 9);
        self.rdfi_identification = MoovIoAchConverters.parse_string_field(&s[..8]);
        self.check_digit = MoovIoAchConverters.parse_string_field(&s[8..9]);
        self
    }
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}, CheckDigit: {}",
            self.rdfi_identification, self.check_digit
        )
    }
}

fn main() {
    let mut entry = MoovIoAchAdvEntryDetail {
        rdfi_identification: String::new(),
        check_digit: String::new(),
        // ... other fields
    };
    let rdfi = "123456789";
    println!("{}", entry.set_rdfi(rdfi));
}
