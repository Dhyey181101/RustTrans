

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchEntryDetail {
    identifer_number: String,
    // ... other fields elided ...
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn shr_document_reference_number_field(&self) -> String {
        self.string_field(&self.identifer_number[4..15], 11)
    }

    fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zeros(m);
        format!("{}{}", pad, s)
    }
}

fn get_zeros(n: usize) -> String {
    let mut out = HashMap::new();
    for i in 0..=n {
        out.insert(i, "0".repeat(i));
    }
    out[&n].to_string()
}

impl fmt::Display for MoovIoAchEntryDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement formatting for MoovIoAchEntryDetail struct here
        write!(f, "...") // Placeholder
    }
}

