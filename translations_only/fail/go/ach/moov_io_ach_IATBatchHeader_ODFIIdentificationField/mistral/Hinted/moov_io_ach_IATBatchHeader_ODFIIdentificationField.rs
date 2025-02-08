

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchIatBatchHeader {
    odfi_identification: String,
    // ... other fields ...
}

struct MoovIoAchConverters {
    max: usize,
}

impl MoovIoAchConverters {
    fn string_field(&self, s: String) -> String {
        let ln = s.len();
        if ln > self.max {
            s[..self.max].to_string()
        } else {
            let m = self.max - ln;
            let pad = get_zeros(m);
            pad + &s
        }
    }
}

impl MoovIoAchIatBatchHeader {
    fn new() -> (MoovIoAchIatBatchHeader, MoovIoAchConverters) {
        let converters = MoovIoAchConverters { max: 8 };
        let iat_batch_header = MoovIoAchIatBatchHeader {
            odfi_identification: "12345678".to_string(),
            // ... other fields ...
        };
        (iat_batch_header, converters)
    }

    fn odfi_identification_field(&self) -> String {
        let (_, converters) = Self::new();
        converters.string_field(self.odfi_identification.clone())
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = String::new();
    for _ in 0..n {
        out.push(ZERO);
    }
    out
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ODFIIdentification: {}, ...",
            self.odfi_identification
        )
    }
}

fn main() {
    let (iat_batch_header, _) = MoovIoAchIatBatchHeader::new();

    println!("{}", iat_batch_header);
}

