
use std::collections::HashMap;

pub struct MoovIoAchAddenda05 {
    pub entry_detail_sequence_number: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAddenda05 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters {}
            .numeric_field(self.entry_detail_sequence_number, 7)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m);
            return pad + &s;
        }
    }
}

fn moov_io_ach_string_zeros(max: u32) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, "0".repeat(i as usize));
    }
    return out[&max].to_string();
}
