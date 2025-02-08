

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = self.get_pad_string(m);
        pad.chars().rev().chain(s.chars()).collect()
    }

    fn get_pad_string(&self, n: usize) -> String {
        let mut map = HashMap::new();
        for i in 0..94 {
            map.insert(i, "0".repeat(i));
        }

        if let Some(pad) = map.get(&n) {
            return pad.clone();
        }

        String::from("".repeat(n))
    }
}

#[derive(Default)]
struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    // ... other fields
}

impl MoovIoAchIatEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        Self::string_field(&self.rdfi_identification, 8)
    }

    fn string_field(s: &str, max: usize) -> String {
        let c = MoovIoAchConverters;
        c.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement formatting for MoovIoAchIatEntryDetail struct
        write!(
            f,
            "RDFIIdentification: {}",
            self.rdfi_identification_field()
        )
    }
}

