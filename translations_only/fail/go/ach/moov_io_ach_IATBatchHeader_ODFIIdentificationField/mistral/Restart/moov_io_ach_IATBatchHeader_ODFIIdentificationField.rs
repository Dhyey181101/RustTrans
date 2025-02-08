

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
        self.string_field(self.odfi_identification.clone(), 8)
    }

    fn string_field(&self, s: String, max: usize) -> String {
        let ln = s.chars().count();
        let m = max.saturating_sub(ln);
        if m > 0 {
            let pad = MoovIoAchConverters::get_zeros(m);
            pad + &s
        } else {
            s
        }
    }
}

impl MoovIoAchConverters {
    fn get_zeros(n: usize) -> String {
        let mut zeros = String::with_capacity(n);
        for _ in 0..n {
            zeros.push(ZERO);
        }
        zeros
    }
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

fn main() {
    let iat_batch_header = MoovIoAchIatBatchHeader {
        odfi_identification: "12345678".to_string(),
        // ... other fields ...
    };
    println!("{}", iat_batch_header);
}

