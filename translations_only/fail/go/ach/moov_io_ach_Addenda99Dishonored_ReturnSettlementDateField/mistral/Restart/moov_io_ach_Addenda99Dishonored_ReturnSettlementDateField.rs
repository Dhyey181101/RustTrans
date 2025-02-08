

use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.len();
        if ln > max {
            s[..max].to_string()
        } else {
            let m = max - ln;
            let pad = self.get_pad_string(m);
            format!("{}{}", pad, s)
        }
    }

    fn get_pad_string(&self, count: usize) -> String {
        let mut out = HashMap::new();
        for i in 0..=count {
            out.insert(i, "0".repeat(i));
        }
        out[&count].clone()
    }
}

#[derive(Default)]
struct MoovIoAchAddenda99Dishonored {
    return_settlement_date: String,
    // ... other fields and implementations ...
    moov_io_ach_converters: MoovIoAchConverters,
}

impl MoovIoAchAddenda99Dishonored {
    fn return_settlement_date_field(&self) -> String {
        self.string_field(&self.return_settlement_date, 3)
    }

    fn string_field(&self, s: &str, max: usize) -> String {
        self.moov_io_ach_converters.string_field(s, max)
    }
}

impl fmt::Display for MoovIoAchAddenda99Dishonored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ... implement display ...
        Ok(())
    }
}

