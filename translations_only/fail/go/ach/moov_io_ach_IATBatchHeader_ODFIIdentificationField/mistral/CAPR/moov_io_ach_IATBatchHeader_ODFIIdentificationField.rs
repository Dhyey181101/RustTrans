

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchIatBatchHeader {
    odfi_identification: String,
    // ... other fields ...
}

struct MoovIoAchConverters;

impl MoovIoAchIatBatchHeader {
    fn odfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.odfi_identification, 8)
    }
}

impl MoovIoAchConverters {
    fn string_field(s: &String, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let pad = get_zeros(m);
        pad + s
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].clone()
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ODFIIdentification: {}, ...",
            self.odfi_identification_field()
        )
    }
}

