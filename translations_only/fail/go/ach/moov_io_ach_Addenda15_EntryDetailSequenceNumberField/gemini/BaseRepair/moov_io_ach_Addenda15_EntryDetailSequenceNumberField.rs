
use std::collections::HashMap;
use std::fmt::Write;

pub struct MoovIoAchAddenda15 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda15 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        let mut pad = String::new();
        let m = (7 - self.entry_detail_sequence_number.to_string().len() as u32) as usize;
        for _ in 0..m {
            write!(&mut pad, "0").unwrap();
        }
        format!("{}{}", pad, self.entry_detail_sequence_number)
    }
}
