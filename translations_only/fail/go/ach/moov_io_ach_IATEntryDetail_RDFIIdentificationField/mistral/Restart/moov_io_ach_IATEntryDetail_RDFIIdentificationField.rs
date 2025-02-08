

use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = Self::get_pad_string(m);
            pad + s
        }
    }

    fn get_pad_string(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].clone()
    }
}

#[derive(Default)]
struct MoovIoAchIatEntryDetail {
    rdfi_identification: String,
    // ... other fields
}

impl MoovIoAchIatEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        self.string_field(&self.rdfi_identification, 8)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        MoovIoAchConverters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchIatEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RDFIIdentification: {}" ,
            self.rdfi_identification_field()
        )
    }
}

fn main() {
    let iat_ed = MoovIoAchIatEntryDetail {
        rdfi_identification: "123456789".to_string(),
        // ... other fields
    };

    println!("{}", iat_ed);
}

