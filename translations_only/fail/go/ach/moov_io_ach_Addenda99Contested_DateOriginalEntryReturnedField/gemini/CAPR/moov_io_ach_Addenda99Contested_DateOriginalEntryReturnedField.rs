
use std::collections::HashMap;

pub struct MoovIoAchAddenda99Contested {
    pub date_original_entry_returned: String,
}

impl MoovIoAchAddenda99Contested {
    pub fn date_original_entry_returned_field(&self) -> &str {
        &self.date_original_entry_returned
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn string_field(&self, s: &str, max: usize) -> String {
        let ln = s.chars().count();
        if ln > max {
            return s[..max].to_string();
        }

        let m = max - ln;
        let binding = moov_io_ach_string_zeros();
        let pad = binding.get(&m).unwrap();
        format!("{}{}", pad, s)
    }
}

fn moov_io_ach_string_zeros() -> HashMap<usize, String> {
    let mut out = HashMap::new();
    for i in 0..94 {
        out.insert(i, String::from("0").repeat(i));
    }
    out
}
