
use std::collections::HashMap;
use std::fmt::Write;

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
        let l = s.len() as u32;
        if l > max {
            return s[(l - max) as usize..].to_string();
        } else {
            let m = max - l;
            let mut pad = String::new();
            for _ in 0..m {
                write!(&mut pad, "0").unwrap();
            }
            return pad + &s;
        }
    }
}

pub fn moov_io_ach_populate_map(max: i32, zero: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    for i in 0..max {
        let mut pad = String::new();
        for _ in 0..i {
            write!(&mut pad, "{}", zero).unwrap();
        }
        out.insert(i, pad);
    }
    out
}
