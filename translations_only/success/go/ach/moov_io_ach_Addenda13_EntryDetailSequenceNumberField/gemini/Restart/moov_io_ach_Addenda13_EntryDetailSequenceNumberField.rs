
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct MoovIoAchAddenda13 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda13 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters::numeric_field(self.entry_detail_sequence_number, 7)
    }
}

pub struct MoovIoAchConverters;

impl MoovIoAchConverters {
    pub fn numeric_field(n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros.get(&m).unwrap();
            return format!("{}{}", pad, s);
        }
    }
}

lazy_static! {
    static ref moov_io_ach_string_zeros: HashMap<u32, String> = {
        let mut out = HashMap::new();
        for i in 0..94 {
            out.insert(i, "0".repeat(i as usize));
        }
        out
    };
}
