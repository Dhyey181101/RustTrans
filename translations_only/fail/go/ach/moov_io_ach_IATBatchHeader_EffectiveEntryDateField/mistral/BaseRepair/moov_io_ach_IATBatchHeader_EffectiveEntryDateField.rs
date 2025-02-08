

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
    // ... other fields elided ...
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters {
    effective_entry_date: String,
    zeros: HashMap<usize, String>,
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date(&self) -> String {
        self.string_field(&self.converters.effective_entry_date, 6) // YYMMDD
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = self.converters.zeros.get(&m).unwrap().to_string();
        pad + s
    }
}

impl MoovIoAchConverters {
    fn new(effective_entry_date: String) -> Self {
        let mut zeros = HashMap::new();
        for i in 0..=10 {
            zeros.insert(i, "0".repeat(i));
        }
        MoovIoAchConverters {
            effective_entry_date,
            zeros,
        }
    }
}

impl MoovIoAchIatBatchHeader {
    fn new() -> Self {
        MoovIoAchIatBatchHeader {
            effective_entry_date: String::new(),
            converters: MoovIoAchConverters::new(String::new()),
        }
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        Ok(())
    }
}

