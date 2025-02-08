
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Addenda14 {
    pub entry_detail_sequence_number: i32,
}

impl Addenda14 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        self.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl Addenda14 {
    fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max as usize - s.len();
            let pad = &STRING_ZEROS[&m];
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref STRING_ZEROS: HashMap<usize, String> = (0..94).map(|i| (i, "0".repeat(i))).collect();
}
