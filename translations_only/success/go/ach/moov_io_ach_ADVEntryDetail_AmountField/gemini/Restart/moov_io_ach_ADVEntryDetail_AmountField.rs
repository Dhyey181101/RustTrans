
use std::collections::HashMap;

pub struct MoovIoAchAdvEntryDetail {
    pub amount: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAdvEntryDetail {
    pub fn amount_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.amount, 12)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m as i32);
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_string_zeros(max: i32) -> String {
    let mut out = String::new();
    for _ in 0..max {
        out.push('0');
    }
    out
}
