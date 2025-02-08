

use std::collections::HashMap;

const ZEROS: &str = "0";

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
    // ... other fields elided ...
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date(&self) -> String {
        MoovIoAchConverters::string_field(self, self.effective_entry_date.as_str(), 6) // YYMMDD
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(header: &MoovIoAchIatBatchHeader, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = get_zeros(m);
        format!("{}{}", pad, s)
    }
}

fn get_zeros(n: usize) -> String {
    let mut map: HashMap<usize, Box<str>> = HashMap::new();
    for i in 0..100 {
        let zeros = "0".repeat(i);
        map.insert(i, Box::from(zeros));
    }
    (*map.get(&n).unwrap_or(&Box::from(ZEROS))).to_string()
}

fn main() {}

