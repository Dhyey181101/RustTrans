

use std::collections::HashMap;

struct MoovIoAchAddenda99 {
    original_dfi: String,
}

impl MoovIoAchAddenda99 {
    fn original_dfi_field(&self) -> String {
        Converters::string_field(&self.original_dfi, 8)
    }
}

struct Converters;

impl Converters {
    fn string_field(s: &String, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            s[..max as usize].to_string()
        } else {
            let mut pad = String::new();
            for _ in 0..(max - ln) {
                pad.push('0');
            }
            format!("{}{}", pad, s)
        }
    }
}

