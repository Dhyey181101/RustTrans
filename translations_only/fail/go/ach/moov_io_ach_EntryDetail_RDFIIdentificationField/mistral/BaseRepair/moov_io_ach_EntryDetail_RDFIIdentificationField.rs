

use std::collections::HashMap;
use lazy_static::lazy_static;

const ZERO: &str = "0";

struct MoovIoAchEntryDetail {
    rdfi_identification: String,
    // ... other fields omitted for brevity
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        self.string_field(&self.rdfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().filter(|c| c.is_whitespace()).count() as u32;
        if ln > max {
            return s[..(max as usize)].to_string();
        }

        let m = (max - ln) as usize;
        let pad = match MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            Some(p) => p.to_string(),
            None => "0".repeat(m),
        };

        pad + s
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = {
        let mut out = HashMap::new();
        for i in 0..100 {
            out.insert(i, ZERO.repeat(i));
        }
        out
    };
}

fn main() {
    // ...
}

