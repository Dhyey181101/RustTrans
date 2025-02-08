
use std::collections::HashMap;

pub struct MoovIoAchAddenda05 {
    pub entry_detail_sequence_number: i32,
}

impl MoovIoAchAddenda05 {
    pub fn entry_detail_sequence_number_field(&self) -> String {
        MoovIoAchConverters {}.numeric_field(self.entry_detail_sequence_number, 7)
    }
}

pub struct MoovIoAchConverters {}

impl MoovIoAchConverters {
    pub fn numeric_field(&self, n: i32, max: u32) -> String {
        let s = n.to_string();
        if s.len() as u32 > max {
            return s[s.len() - max as usize..].to_string();
        } else {
            let m = (max - s.len() as u32) as usize;
            let mut pad = String::new();
            for _ in 0..m {
                pad.push('0');
            }
            return pad + &s;
        }
    }
}

pub fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut s = String::new();
        for _ in 0..i {
            s.push(zero.chars().next().unwrap());
        }
        out.insert(i, s);
    }
    out
}
