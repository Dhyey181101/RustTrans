
use std::collections::HashMap;

struct MoovIoAchAddenda99Dishonored {
    original_receiving_dfi_identification: String,
    moov_io_ach_converters: MoovIoAchConverters,
}

struct MoovIoAchConverters;

impl MoovIoAchAddenda99Dishonored {
    fn original_receiving_dfi_identification_field(&self) -> String {
        string_field(&self.original_receiving_dfi_identification, 8)
    }
}

fn string_field(s: &String, max: u32) -> String {
    let len = s.len() as u32;
    if len > max {
        s[..max as usize].to_string()
    } else {
        let mut pad = String::new();
        for _ in 0..(max - len) {
            pad.push('0');
        }
        pad + s
    }
}

