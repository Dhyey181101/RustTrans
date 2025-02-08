
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct MoovIoAchAddenda10 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda10 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        let s = self.entry_detail_sequence_number.to_string();
        let m = 7 - s.len();
        let pad = STRING_ZEROS.get(&m).unwrap_or(&"0".repeat(m));
        let binding = "0".repeat(m);
        let pad = STRING_ZEROS.get(&m).unwrap_or(&binding);
        format!("{}{}", pad, s)
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> =
        (0..94).map(|i| (i, "0".repeat(i))).collect();
}
