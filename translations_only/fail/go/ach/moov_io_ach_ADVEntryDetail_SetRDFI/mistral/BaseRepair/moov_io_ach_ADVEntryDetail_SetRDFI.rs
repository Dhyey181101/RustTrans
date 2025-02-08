

use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let len = s.len();
        if len > max {
            s[..max].to_string()
        } else {
            let m = max - len;
            let pad = self.get_zeros(m);
            String::from_utf8_lossy(pad).to_string() + s
        }
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }

    fn get_zeros(&self, n: usize) -> &'static [u8] {
        &ZEROS[..n]
    }
}

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields ...
}

impl MoovIoAchAdvEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let s = self.string_field(rdfi, 9);
        self.rdfi_identification = self.parse_string_field(&s[..8]);
        self.check_digit = self.parse_string_field(&s[8..9]);
        self
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }

    fn parse_string_field(&self, r: &str) -> String {
        MoovIoAchConverters.parse_string_field(r)
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
        // ... other fields ...
    };

    let rdfi = "111000222";
    entry.set_rdfi(rdfi);
    println!("{}", entry);
}

