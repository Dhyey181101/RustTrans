
use std::collections::HashMap;

pub struct MoovIoAchAddenda18 {
    pub sequence_number: i32,
}

impl MoovIoAchAddenda18 {
    pub fn sequence_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.sequence_number, 4)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_populate_map(m as i32, "0");
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_populate_map(max: i32, zero: &str) -> String {
    let mut out = String::new();
    for _ in 0..max {
        out.push_str(zero);
    }
    out
}
