
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MOOV_IO_ACH_STRING_ZEROS: Lazy<HashMap<i32, String>> = Lazy::new(|| HashMap::new());

struct MoovIoAchFileControl {
    entry_addenda_count: i32,
}

struct MoovIoAchConverters;

impl MoovIoAchFileControl {
    fn entry_addenda_count_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_addenda_count, 8)
    }
}

impl MoovIoAchConverters {
    fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            s[s.len() - max as usize..].to_string()
        } else {
            let mut result = String::with_capacity(max as usize);
            result.push_str(&"0".repeat(max as usize - s.len()));
            result.push_str(&s);
            result
        }
    }
}
