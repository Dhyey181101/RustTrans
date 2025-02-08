

use std::collections::HashMap;

struct MoovIoAchAddenda98Refused {
    original_dfi: String,
}

impl MoovIoAchAddenda98Refused {
    fn original_dfi_field(&self) -> String {
        string_field(&self.original_dfi, 8)
    }
}

struct MoovIoAchConverters;

fn string_field(s: &str, max: u32) -> String {
    let ln = s.chars().count() as u32;
    if ln > max {
        s[..max as usize].to_string()
    } else {
        format!("{}{}", "0".repeat(max as usize - ln as usize), s)
    }
}

