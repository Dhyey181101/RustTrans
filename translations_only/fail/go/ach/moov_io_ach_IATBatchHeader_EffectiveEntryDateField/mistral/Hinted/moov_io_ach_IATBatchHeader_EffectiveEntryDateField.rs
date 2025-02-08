

use std::collections::HashMap;
use std::fmt;

const ZERO: &str = "0";

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
    // ... other fields elided ...
    converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date(&self) -> String {
        self.string_field(&self.effective_entry_date, 6) // YYMMDD
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            return s[..max].to_string();
        }

        // Pad with preallocated string
        let m = max - ln;
        let pad = MoovIoAchStringZeros::get(m).to_string();
        pad + s
    }
}

struct MoovIoAchStringZeros {
    data: HashMap<usize, String>,
}

impl MoovIoAchStringZeros {
    fn new(max: usize, zero: &str) -> Box<Self> {
        let mut data = HashMap::new();
        for i in 0..=max {
            data.insert(i, "0".repeat(i));
        }
        Box::new(MoovIoAchStringZeros { data })
    }

    fn get(m: usize) -> String {
        MoovIoAchStringZeros::new(100, ZERO).data[&m].clone()
    }
}

impl fmt::Display for MoovIoAchIatBatchHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... formatting elided ...
        write!(
            f,
            "EffectiveEntryDate: {}\n",
            self.effective_entry_date()
        )
    }
}

fn main() {
    let iat_batch_header = MoovIoAchIatBatchHeader {
        effective_entry_date: "210205".to_string(),
        // ... other fields initialized ...
        converters: MoovIoAchConverters {},
    };

    println!("{}", iat_batch_header);
}

