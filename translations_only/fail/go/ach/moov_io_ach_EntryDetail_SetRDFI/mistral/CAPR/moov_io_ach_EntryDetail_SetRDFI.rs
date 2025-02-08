
use std::collections::HashMap;
use std::fmt;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            s[0..max as usize].to_string()
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

fn get_zeros(n: usize) -> String {
    let mut zeros = String::new();
    for _ in 0..n {
        zeros.push('0');
    }
    zeros
}

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields
}

impl MoovIoAchEntryDetail {
    fn set_rdfi(&mut self, rdfi: &str) -> &mut Self {
        let converters = &MoovIoAchConverters;
        let s = converters.string_field(rdfi, 9);
        self.rdfi_identification = converters.parse_string_field(&s[..8]);
        self.check_digit = converters.parse_string_field(&s[8..9]);
        self
    }
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
    let mut ed = Box::new(MoovIoAchEntryDetail {
        rdfi_identification: String::new(),
        check_digit: String::new(),
        // ... other fields
    });
    let rdfi = "123456789";
    ed.set_rdfi(rdfi);
    println!("{}", ed);
}
