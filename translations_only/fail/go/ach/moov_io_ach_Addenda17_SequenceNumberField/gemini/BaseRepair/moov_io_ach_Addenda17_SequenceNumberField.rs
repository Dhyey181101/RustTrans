
use std::collections::HashMap;

pub struct MoovIoAchAddenda17 {
    pub sequence_number: i32,
}

pub struct MoovIoAchConverters {}

impl MoovIoAchAddenda17 {
    pub fn sequence_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.sequence_number, 4)
    }
}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[(s.len() - max as usize)..].to_string();
        } else {
            let m = max - s.len() as u32;
            let pad = moov_io_ach_string_zeros(m);
            return format!("{}{}", pad, s);
        }
    }
}

fn moov_io_ach_string_zeros(max: u32) -> String {
    let mut out = HashMap::new();
    for i in 0..max {
        out.insert(i, String::from("0").repeat(i as usize));
    }
    out[&max].clone()
}
