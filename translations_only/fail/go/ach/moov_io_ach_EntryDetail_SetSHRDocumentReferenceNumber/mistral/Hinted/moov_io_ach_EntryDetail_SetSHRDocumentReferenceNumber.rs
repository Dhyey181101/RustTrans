

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
    fn set_shr_document_reference_number(&mut self, s: String) {
        self.identification_number = self
            .identification_number
            .clone()
            + &self.string_field(s, 11);
    }

    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        if let Some(pad) = MOOV_IO_ACH_STRING_ZEROS.get(&m) {
            return pad.to_string() + &s;
        }

        "0".repeat(m) + &s
    }
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting implementation ...
        Ok(())
    }
}

const MOOV_IO_ACH_MAX: usize = 94;

fn create_string_zeros_map() -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for i in 0..MOOV_IO_ACH_MAX {
        map.insert(i, "0".repeat(i));
    }
    map
}

lazy_static! {
    static ref MOOV_IO_ACH_STRING_ZEROS: HashMap<usize, String> = create_string_zeros_map();
}

fn main() {
    // ... example usage ...
}

