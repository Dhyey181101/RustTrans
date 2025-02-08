

use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;

struct MoovIoAchEntryDetail {
    identification_number: String,
    // ... other fields ...
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: &str) {
        self.identification_number = self
            .identification_number
            .clone()
            + &self.string_field(s.to_string(), 11);
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.clone() + &s;
        }

        "0".repeat(m) + &s
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement formatting for MoovIoAchEntryDetail here
        write!(f, "{}", self.identification_number)
    }
}

const MOOV_IO_ACH_MAX: usize = 94;
struct MOOV_IO_ACH_STRING_ZEROS_MAP {
    map: HashMap<usize, String>,
}

impl MOOV_IO_ACH_STRING_ZEROS_MAP {
    fn new() -> MOOV_IO_ACH_STRING_ZEROS_MAP {
        let mut map = HashMap::new();
        for i in 0..MOOV_IO_ACH_MAX {
            map.insert(i, "0".repeat(i));
        }
        MOOV_IO_ACH_STRING_ZEROS_MAP { map }
    }

    fn get(&self, k: &usize) -> Option<&String> {
        self.map.get(k)
    }
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: MOOV_IO_ACH_STRING_ZEROS_MAP =
        MOOV_IO_ACH_STRING_ZEROS_MAP::new();
}

fn main() {
    let mut ed = MoovIoAchEntryDetail {
        identification_number: "existing_number".to_string(),
        // ... other fields ...
        converters: MoovIoAchConverters,
    };

    ed.set_shr_document_reference_number("new_number");
    println!("{}", ed);
}

