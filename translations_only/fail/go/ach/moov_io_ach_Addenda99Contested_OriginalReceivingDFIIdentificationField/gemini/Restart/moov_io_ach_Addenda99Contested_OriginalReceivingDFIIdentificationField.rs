
use std::collections::HashMap;

pub struct MoovIoAchAddenda99Contested {
    pub original_receiving_dfi_identification: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn original_receiving_dfi_identification(&self) -> &str {
        &self.original_receiving_dfi_identification
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: u32) -> String {
        let ln = s.chars().count() as u32;
        if ln > max {
            return s[..max as usize].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        return format!("{}{}", pad, s);
    }
}

fn moov_io_ach_string_zeros() -> HashMap<u32, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from("0").repeat(i as usize));
    }
    out
}
