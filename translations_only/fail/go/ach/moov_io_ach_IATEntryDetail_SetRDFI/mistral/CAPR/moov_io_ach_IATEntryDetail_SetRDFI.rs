

use std::collections::HashMap;
use std::fmt;

const MAX_MAP_SIZE: usize = 94;

struct MoovIoAchIATEntryDetail {
    rdfi_identification: String,
    check_digit: String,
    // ... other fields ...
}

impl MoovIoAchIATEntryDetail {
    fn set_rdfi(&mut self, rdfi: String) -> &MoovIoAchIATEntryDetail {
        let s = self.string_field(rdfi, 9);
        self.rdfi_identification = self.parse_string_field(&s[..8]);
        self.check_digit = self.parse_string_field(&s[8..9]);
        self
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchConverters::get_zeros(m);
        format!("{}{}", pad, s)
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn get_zeros(n: usize) -> String {
        let mut zeros = HashMap::new();
        for i in 0..MAX_MAP_SIZE {
            zeros.insert(i, "0".repeat(i));
        }

        if let Some(value) = zeros.get(&n) {
            return value.clone();
        }

        "0".repeat(n)
    }
}

impl fmt::Display for MoovIoAchIATEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}, CheckDigit: {}",
            self.rdfi_identification, self.check_digit
        )
    }
}

