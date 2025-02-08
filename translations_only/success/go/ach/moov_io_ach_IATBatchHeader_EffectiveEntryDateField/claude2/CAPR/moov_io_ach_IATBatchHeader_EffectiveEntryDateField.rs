
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRINGZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchIatBatchHeader {
    effective_entry_date: String,
}

impl MoovIoAchIatBatchHeader {
    fn effective_entry_date_field(&self) -> String {
        MoovIoAchConverters::string_field(&self.effective_entry_date, 6)
    }
}

struct MoovIoAchConverters;

impl MoovIoAchConverters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            pad + s
        }
    }
}
