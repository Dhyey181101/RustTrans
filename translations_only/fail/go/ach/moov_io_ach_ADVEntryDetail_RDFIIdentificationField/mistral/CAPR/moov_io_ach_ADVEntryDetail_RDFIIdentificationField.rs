

use std::collections::HashMap;
use std::fmt;

const ZEROS: &[u8] = b"0000000000000000000000000000000000000000000000000000000000000000";

struct MoovIoAchAdvEntryDetail {
    rdfi_identification: String,
    // ... other fields elided ...
}

impl MoovIoAchAdvEntryDetail {
    fn rdfi_identification_field(&self) -> String {
        MoovIoAchConverters::string_field(self, &self.rdfi_identification, 8)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(entry: &MoovIoAchAdvEntryDetail, s: &str, max: usize) -> String {
        let len = s.len();
        if len > max {
            s[..max].to_string()
        } else {
            let zeros = &ZEROS[..(max - len)];
            if let Ok(s) = std::str::from_utf8(zeros) {
                s.to_string() + s
            } else {
                unreachable!()
            }
        }
    }
}

fn populate_map(max: usize, zero: &str) -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i));
    }
    out
}

impl fmt::Display for MoovIoAchAdvEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MoovIoAchAdvEntryDetail(\n    rdfi_identification: {:?},\n)",
            self.rdfi_identification
        )
    }
}

