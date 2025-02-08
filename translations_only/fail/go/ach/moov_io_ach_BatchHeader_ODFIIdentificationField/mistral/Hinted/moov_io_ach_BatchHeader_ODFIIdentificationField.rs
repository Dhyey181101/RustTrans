

use std::collections::HashMap;
use std::fmt;

const ZEROS: &str = "0";

struct MoovIoAchBatchHeader {
    odfi_identification: String,
    // ... other fields elided for brevity ...
}

impl MoovIoAchBatchHeader {
    fn odfi_identification_field(&self) -> String {
        self.string_field(self.odfi_identification.clone(), 8)
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = MoovIoAchConverters::get_zeros(m);
        pad + &s
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn get_zeros(n: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=n {
            out.insert(i, "0".repeat(i));
        }
        out[&n].to_string()
    }
}

impl fmt::Display for MoovIoAchBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ODFIIdentification: {}, \
             BatchNumber: ...",
            self.odfi_identification_field()
        )
    }
}

fn main() {
    let bh = MoovIoAchBatchHeader {
        odfi_identification: "123456789".to_string(),
        // ... other fields elided for brevity ...
    };
    println!("{}", bh);
}

