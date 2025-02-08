

use std::collections::HashMap;
use std::fmt;

struct MoovIoAchEntryDetail {
    identification_number: String,
    // ... other fields ...
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchEntryDetail {
    fn set_shr_document_reference_number(&mut self, s: String) {
        self.identification_number = format!(
            "{}-{}",
            self.identification_number,
            self.converters.string_field(s, 11)
        );
    }
}

impl MoovIoAchConverters {
    fn string_field(&self, s: String, max: u32) -> String {
        let ln = s.len() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = (max - ln) as usize;
        let pad = get_zeros(m);
        pad + &s
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
        // Implement formatting for MoovIoAchEntryDetail here
        write!(f, "{}", self.identification_number)
    }
}

