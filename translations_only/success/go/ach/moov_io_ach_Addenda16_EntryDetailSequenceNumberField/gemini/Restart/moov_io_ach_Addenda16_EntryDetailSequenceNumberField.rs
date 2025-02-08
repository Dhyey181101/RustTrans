
use std::collections::HashMap;

pub struct MoovIoAchAddenda16 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda16 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters {}
            .numeric_field(self.entry_detail_sequence_number, 7)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let pad = moov_io_ach_string_zeros(m as usize);
            return pad + &s;
        }
    }
}

fn moov_io_ach_string_zeros(m: usize) -> String {
    let mut out = String::with_capacity(m);
    for _ in 0..m {
        out.push('0');
    }
    out
}
