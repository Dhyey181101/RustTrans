

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
            return s[..(max as usize)].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchConverters::new().get_zero_string(m);
        pad + &s
    }

    fn parse_string_field(&self, r: &str) -> String {
        r.trim().to_string()
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn new() -> MoovIoAchConverters {
        MoovIoAchConverters {}
    }

    fn get_zero_string(&self, n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..MAX_MAP_SIZE {
            map.insert(i, "0".repeat(i));
        }
        map[&n].clone()
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

