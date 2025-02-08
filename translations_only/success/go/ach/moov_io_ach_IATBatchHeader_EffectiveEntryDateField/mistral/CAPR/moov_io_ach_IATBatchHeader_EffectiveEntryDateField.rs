

use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;
use std::boxed::Box;

const ZEROS: &str = "0";

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
    // ... other fields elided ...
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date(&self) -> String {
        MoovIoAchConverters::new().string_field(&self.effective_entry_date, 6) // YYMMDD
    }
}

struct MoovIoAchConverters {
    zero_padding: HashMap<usize, String>,
}

impl MoovIoAchConverters {
    fn new() -> Box<Self> {
        let mut map = HashMap::new();
        for i in 0..100 {
            let zeros = "0".repeat(i);
            map.insert(i, zeros);
        }
        Box::new(MoovIoAchConverters { zero_padding: map })
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = self.zero_padding.get(&m).unwrap().to_string();
        format!("{}{}", pad, s)
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // ... implementation elided ...
        Ok(())
    }
}

